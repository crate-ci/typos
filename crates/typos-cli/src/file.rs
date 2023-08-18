use bstr::ByteSlice;
use std::io::Read;
use std::io::Write;

use crate::report;

pub trait FileChecker: Send + Sync {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct Typos;

impl FileChecker for Typos {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        if policy.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                for typo in typos::check_str(file_name, policy.tokenizer, policy.dict) {
                    let msg = report::Typo {
                        context: Some(report::PathContext { path }.into()),
                        buffer: std::borrow::Cow::Borrowed(file_name.as_bytes()),
                        byte_offset: typo.byte_offset,
                        typo: typo.typo.as_ref(),
                        corrections: typo.corrections,
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        if policy.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !policy.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut accum_line_num = AccumulateLineNum::new();
                for typo in check_bytes(&buffer, policy) {
                    let line_num = accum_line_num.line_num(&buffer, typo.byte_offset);
                    let (line, line_offset) = extract_line(&buffer, typo.byte_offset);
                    let msg = report::Typo {
                        context: Some(report::FileContext { path, line_num }.into()),
                        buffer: std::borrow::Cow::Borrowed(line),
                        byte_offset: line_offset,
                        typo: typo.typo.as_ref(),
                        corrections: typo.corrections,
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixTypos;

impl FileChecker for FixTypos {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        if policy.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !policy.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut fixes = Vec::new();
                let mut accum_line_num = AccumulateLineNum::new();
                for typo in check_bytes(&buffer, policy) {
                    if is_fixable(&typo) {
                        fixes.push(typo.into_owned());
                    } else {
                        let line_num = accum_line_num.line_num(&buffer, typo.byte_offset);
                        let (line, line_offset) = extract_line(&buffer, typo.byte_offset);
                        let msg = report::Typo {
                            context: Some(report::FileContext { path, line_num }.into()),
                            buffer: std::borrow::Cow::Borrowed(line),
                            byte_offset: line_offset,
                            typo: typo.typo.as_ref(),
                            corrections: typo.corrections,
                        };
                        reporter.report(msg.into())?;
                    }
                }
                if !fixes.is_empty() || path == std::path::Path::new("-") {
                    let buffer = fix_buffer(buffer, fixes.into_iter());
                    write_file(path, content_type, buffer, reporter)?;
                }
            }
        }

        // Ensure the above write can happen before renaming the file.
        if policy.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                let mut fixes = Vec::new();
                for typo in typos::check_str(file_name, policy.tokenizer, policy.dict) {
                    if is_fixable(&typo) {
                        fixes.push(typo.into_owned());
                    } else {
                        let msg = report::Typo {
                            context: Some(report::PathContext { path }.into()),
                            buffer: std::borrow::Cow::Borrowed(file_name.as_bytes()),
                            byte_offset: typo.byte_offset,
                            typo: typo.typo.as_ref(),
                            corrections: typo.corrections,
                        };
                        reporter.report(msg.into())?;
                    }
                }
                if !fixes.is_empty() {
                    let file_name = file_name.to_owned().into_bytes();
                    let new_name = fix_buffer(file_name, fixes.into_iter());
                    let new_name =
                        String::from_utf8(new_name).expect("corrections are valid utf-8");
                    let new_path = path.with_file_name(new_name);
                    std::fs::rename(path, new_path)?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiffTypos;

impl FileChecker for DiffTypos {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let mut content = Vec::new();
        let mut new_content = Vec::new();
        if policy.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !policy.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut fixes = Vec::new();
                let mut accum_line_num = AccumulateLineNum::new();
                for typo in check_bytes(&buffer, policy) {
                    if is_fixable(&typo) {
                        fixes.push(typo.into_owned());
                    } else {
                        let line_num = accum_line_num.line_num(&buffer, typo.byte_offset);
                        let (line, line_offset) = extract_line(&buffer, typo.byte_offset);
                        let msg = report::Typo {
                            context: Some(report::FileContext { path, line_num }.into()),
                            buffer: std::borrow::Cow::Borrowed(line),
                            byte_offset: line_offset,
                            typo: typo.typo.as_ref(),
                            corrections: typo.corrections,
                        };
                        reporter.report(msg.into())?;
                    }
                }
                if !fixes.is_empty() {
                    new_content = fix_buffer(buffer.clone(), fixes.into_iter());
                    content = buffer
                }
            }
        }

        // Match FixTypos ordering for easy diffing.
        let mut new_path = None;
        if policy.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                let mut fixes = Vec::new();
                for typo in typos::check_str(file_name, policy.tokenizer, policy.dict) {
                    if is_fixable(&typo) {
                        fixes.push(typo.into_owned());
                    } else {
                        let msg = report::Typo {
                            context: Some(report::PathContext { path }.into()),
                            buffer: std::borrow::Cow::Borrowed(file_name.as_bytes()),
                            byte_offset: typo.byte_offset,
                            typo: typo.typo.as_ref(),
                            corrections: typo.corrections,
                        };
                        reporter.report(msg.into())?;
                    }
                }
                if !fixes.is_empty() {
                    let file_name = file_name.to_owned().into_bytes();
                    let new_name = fix_buffer(file_name, fixes.into_iter());
                    let new_name =
                        String::from_utf8(new_name).expect("corrections are valid utf-8");
                    new_path = Some(path.with_file_name(new_name));
                }
            }
        }

        if new_path.is_some() || !content.is_empty() {
            let original_path = path.display().to_string();
            let fixed_path = new_path.as_deref().unwrap_or(path).display().to_string();
            let original_content: Vec<_> = content
                .lines_with_terminator()
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect();
            let fixed_content: Vec<_> = new_content
                .lines_with_terminator()
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect();
            let diff = difflib::unified_diff(
                &original_content,
                &fixed_content,
                original_path.as_str(),
                fixed_path.as_str(),
                "original",
                "fixed",
                0,
            );
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            for line in diff {
                write!(handle, "{}", line)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Identifiers;

impl FileChecker for Identifiers {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        if policy.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                for word in policy.tokenizer.parse_str(file_name) {
                    let msg = report::Parse {
                        context: Some(report::PathContext { path }.into()),
                        kind: report::ParseKind::Identifier,
                        data: word.token(),
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        if policy.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !policy.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut ignores: Option<Ignores> = None;
                for word in policy.tokenizer.parse_bytes(&buffer) {
                    if ignores
                        .get_or_insert_with(|| Ignores::new(&buffer, policy.ignore))
                        .is_ignored(word.span())
                    {
                        continue;
                    }
                    // HACK: Don't look up the line_num per entry to better match the performance
                    // of Typos for comparison purposes.  We don't really get much out of it
                    // anyway.
                    let line_num = 0;
                    let msg = report::Parse {
                        context: Some(report::FileContext { path, line_num }.into()),
                        kind: report::ParseKind::Identifier,
                        data: word.token(),
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Words;

impl FileChecker for Words {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        if policy.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                for word in policy
                    .tokenizer
                    .parse_str(file_name)
                    .flat_map(|i| i.split())
                {
                    let msg = report::Parse {
                        context: Some(report::PathContext { path }.into()),
                        kind: report::ParseKind::Word,
                        data: word.token(),
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        if policy.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !policy.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut ignores: Option<Ignores> = None;
                for word in policy
                    .tokenizer
                    .parse_bytes(&buffer)
                    .flat_map(|i| i.split())
                {
                    if ignores
                        .get_or_insert_with(|| Ignores::new(&buffer, policy.ignore))
                        .is_ignored(word.span())
                    {
                        continue;
                    }
                    // HACK: Don't look up the line_num per entry to better match the performance
                    // of Typos for comparison purposes.  We don't really get much out of it
                    // anyway.
                    let line_num = 0;
                    let msg = report::Parse {
                        context: Some(report::FileContext { path, line_num }.into()),
                        kind: report::ParseKind::Word,
                        data: word.token(),
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FileTypes;

impl FileChecker for FileTypes {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        // Check `policy.binary` first so we can easily check performance of walking vs reading
        if policy.binary {
            let msg = report::FileType::new(path, policy.file_type);
            reporter.report(msg.into())?;
        } else {
            let (_buffer, content_type) = read_file(path, reporter)?;
            if !explicit && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let msg = report::FileType::new(path, policy.file_type);
                reporter.report(msg.into())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FoundFiles;

impl FileChecker for FoundFiles {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        policy: &crate::policy::Policy,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        // Check `policy.binary` first so we can easily check performance of walking vs reading
        if policy.binary {
            let msg = report::File::new(path);
            reporter.report(msg.into())?;
        } else {
            let (_buffer, content_type) = read_file(path, reporter)?;
            if !explicit && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let msg = report::File::new(path);
                reporter.report(msg.into())?;
            }
        }

        Ok(())
    }
}

fn read_file(
    path: &std::path::Path,
    reporter: &dyn report::Report,
) -> Result<(Vec<u8>, content_inspector::ContentType), std::io::Error> {
    let buffer = if path == std::path::Path::new("-") {
        let mut buffer = Vec::new();
        report_result(
            std::io::stdin().read_to_end(&mut buffer),
            Some(path),
            reporter,
        )?;
        buffer
    } else {
        report_result(std::fs::read(path), Some(path), reporter)?
    };

    let content_type = content_inspector::inspect(&buffer);

    let (buffer, content_type) = match content_type {
        content_inspector::ContentType::BINARY |
        // HACK: We don't support UTF-32 yet
        content_inspector::ContentType::UTF_32LE |
        content_inspector::ContentType::UTF_32BE => {
            (buffer, content_inspector::ContentType::BINARY)
        },
        content_inspector::ContentType::UTF_8 |
        content_inspector::ContentType::UTF_8_BOM => {
            (buffer, content_type)
        },
        content_inspector::ContentType::UTF_16LE => {
            // Despite accepting a `String`, decode_to_string_without_replacement` doesn't allocate
            // so to avoid `OutputFull` loops, we're going to assume any UTF-16 content can fit in
            // a buffer twice its size
            let mut decoded = String::with_capacity(buffer.len() * 2);
            let (r, written) = encoding_rs::UTF_16LE.new_decoder_with_bom_removal().decode_to_string_without_replacement(&buffer, &mut decoded, true);
            let decoded = match r {
                encoding_rs::DecoderResult::InputEmpty => Ok(decoded),
                _ => Err(format!("invalid UTF-16LE encoding at byte {} in {}", written, path.display())),
            };
            let buffer = report_result(decoded, Some(path), reporter)?;
            (buffer.into_bytes(), content_type)
        }
        content_inspector::ContentType::UTF_16BE => {
            // Despite accepting a `String`, decode_to_string_without_replacement` doesn't allocate
            // so to avoid `OutputFull` loops, we're going to assume any UTF-16 content can fit in
            // a buffer twice its size
            let mut decoded = String::with_capacity(buffer.len() * 2);
            let (r, written) = encoding_rs::UTF_16BE.new_decoder_with_bom_removal().decode_to_string_without_replacement(&buffer, &mut decoded, true);
            let decoded = match r {
                encoding_rs::DecoderResult::InputEmpty => Ok(decoded),
                _ => Err(format!("invalid UTF-16BE encoding at byte {} in {}", written, path.display())),
            };
            let buffer = report_result(decoded, Some(path), reporter)?;
            (buffer.into_bytes(), content_type)
        },
    };

    Ok((buffer, content_type))
}

fn write_file(
    path: &std::path::Path,
    content_type: content_inspector::ContentType,
    buffer: Vec<u8>,
    reporter: &dyn report::Report,
) -> Result<(), std::io::Error> {
    let buffer = match content_type {
        // HACK: We don't support UTF-32 yet
        content_inspector::ContentType::UTF_32LE | content_inspector::ContentType::UTF_32BE => {
            unreachable!("read_file should prevent these from being passed along");
        }
        content_inspector::ContentType::BINARY
        | content_inspector::ContentType::UTF_8
        | content_inspector::ContentType::UTF_8_BOM => buffer,
        content_inspector::ContentType::UTF_16LE => {
            let buffer = report_result(String::from_utf8(buffer), Some(path), reporter)?;
            if buffer.is_empty() {
                // Error occurred, don't clear out the file
                return Ok(());
            }
            let (encoded, _, replaced) = encoding_rs::UTF_16LE.encode(&buffer);
            assert!(
                !replaced,
                "Coming from UTF-8, UTF-16LE shouldn't do replacements"
            );
            encoded.into_owned()
        }
        content_inspector::ContentType::UTF_16BE => {
            let buffer = report_result(String::from_utf8(buffer), Some(path), reporter)?;
            if buffer.is_empty() {
                // Error occurred, don't clear out the file
                return Ok(());
            }
            let (encoded, _, replaced) = encoding_rs::UTF_16BE.encode(&buffer);
            assert!(
                !replaced,
                "Coming from UTF-8, UTF-16BE shouldn't do replacements"
            );
            encoded.into_owned()
        }
    };

    if path == std::path::Path::new("-") {
        report_result(std::io::stdout().write_all(&buffer), Some(path), reporter)?;
    } else {
        report_result(std::fs::write(path, buffer), Some(path), reporter)?;
    }

    Ok(())
}

fn check_bytes<'a>(
    buffer: &'a [u8],
    policy: &'a crate::policy::Policy<'a, 'a, 'a>,
) -> impl Iterator<Item = typos::Typo<'a>> {
    let mut ignores: Option<Ignores> = None;

    typos::check_bytes(buffer, policy.tokenizer, policy.dict).filter(move |typo| {
        !ignores
            .get_or_insert_with(|| Ignores::new(buffer, policy.ignore))
            .is_ignored(typo.span())
    })
}

fn report_result<T: Default, E: ToString>(
    value: Result<T, E>,
    path: Option<&std::path::Path>,
    reporter: &dyn report::Report,
) -> Result<T, std::io::Error> {
    let buffer = match value {
        Ok(value) => value,
        Err(err) => {
            report_error(err, path, reporter)?;
            Default::default()
        }
    };
    Ok(buffer)
}

fn report_error<E: ToString>(
    err: E,
    path: Option<&std::path::Path>,
    reporter: &dyn report::Report,
) -> Result<(), std::io::Error> {
    let mut msg = report::Error::new(err.to_string());
    msg.context = path.map(|path| report::Context::Path(report::PathContext { path }));
    reporter.report(msg.into())?;
    Ok(())
}

struct AccumulateLineNum {
    line_num: usize,
    last_offset: usize,
}

impl AccumulateLineNum {
    fn new() -> Self {
        Self {
            // 1-indexed
            line_num: 1,
            last_offset: 0,
        }
    }

    fn line_num(&mut self, buffer: &[u8], byte_offset: usize) -> usize {
        assert!(self.last_offset <= byte_offset);
        let slice = &buffer[self.last_offset..byte_offset];
        let newlines = slice.find_iter(b"\n").count();
        let line_num = self.line_num + newlines;
        self.line_num = line_num;
        self.last_offset = byte_offset;
        line_num
    }
}

fn extract_line(buffer: &[u8], byte_offset: usize) -> (&[u8], usize) {
    let line_start = buffer[0..byte_offset]
        .rfind_byte(b'\n')
        // Skip the newline
        .map(|s| s + 1)
        .unwrap_or(0);
    let line = buffer[line_start..]
        .lines()
        .next()
        .expect("should always be at least a line");
    let line_offset = byte_offset - line_start;
    (line, line_offset)
}

fn extract_fix<'t>(typo: &'t typos::Typo<'t>) -> Option<&'t str> {
    match &typo.corrections {
        typos::Status::Corrections(c) if c.len() == 1 => Some(c[0].as_ref()),
        _ => None,
    }
}

fn is_fixable(typo: &typos::Typo<'_>) -> bool {
    extract_fix(typo).is_some()
}

fn fix_buffer(mut buffer: Vec<u8>, typos: impl Iterator<Item = typos::Typo<'static>>) -> Vec<u8> {
    let mut offset = 0isize;
    for typo in typos {
        let fix = extract_fix(&typo).expect("Caller only provides fixable typos");
        let start = ((typo.byte_offset as isize) + offset) as usize;
        let end = start + typo.typo.len();

        buffer.splice(start..end, fix.as_bytes().iter().copied());

        offset += (fix.len() as isize) - (typo.typo.len() as isize);
    }
    buffer
}

pub fn walk_path(
    walk: ignore::Walk,
    checks: &dyn FileChecker,
    engine: &crate::policy::ConfigEngine,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    for entry in walk {
        walk_entry(entry, checks, engine, reporter)?;
    }
    Ok(())
}

pub fn walk_path_parallel(
    walk: ignore::WalkParallel,
    checks: &dyn FileChecker,
    engine: &crate::policy::ConfigEngine,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    let error: std::sync::Mutex<Result<(), ignore::Error>> = std::sync::Mutex::new(Ok(()));
    walk.run(|| {
        Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
            match walk_entry(entry, checks, engine, reporter) {
                Ok(()) => ignore::WalkState::Continue,
                Err(err) => {
                    *error.lock().unwrap() = Err(err);
                    ignore::WalkState::Quit
                }
            }
        })
    });

    error.into_inner().unwrap()
}

fn walk_entry(
    entry: Result<ignore::DirEntry, ignore::Error>,
    checks: &dyn FileChecker,
    engine: &crate::policy::ConfigEngine,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(err) => {
            report_error(err, None, reporter)?;
            return Ok(());
        }
    };
    if crate::config::SUPPORTED_FILE_NAMES
        .iter()
        .any(|n| *n == entry.file_name())
    {
        return Ok(());
    }
    if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
        let explicit = entry.depth() == 0;
        let (path, lookup_path) = if entry.is_stdin() {
            let path = std::path::Path::new("-");
            let cwd = std::env::current_dir().map_err(|err| {
                let kind = err.kind();
                std::io::Error::new(kind, "no current working directory".to_owned())
            })?;
            (path, cwd)
        } else {
            let path = entry.path();
            let abs_path = report_result(path.canonicalize(), Some(path), reporter)?;
            (path, abs_path)
        };
        let policy = engine.policy(&lookup_path);
        checks.check_file(path, explicit, &policy, reporter)?;
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct Ignores {
    blocks: Vec<std::ops::Range<usize>>,
}

impl Ignores {
    fn new(content: &[u8], ignores: &[regex::Regex]) -> Self {
        let mut blocks = Vec::new();
        if let Ok(content) = std::str::from_utf8(content) {
            for ignore in ignores {
                for mat in ignore.find_iter(content) {
                    blocks.push(mat.range());
                }
            }
        }
        Self { blocks }
    }

    fn is_ignored(&self, span: std::ops::Range<usize>) -> bool {
        let start = span.start;
        let end = span.end.saturating_sub(1);
        self.blocks
            .iter()
            .any(|block| block.contains(&start) || block.contains(&end))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fix_simple(line: &str, corrections: Vec<(usize, &'static str, &'static str)>) -> String {
        let line = line.as_bytes().to_vec();
        let corrections = corrections
            .into_iter()
            .map(|(byte_offset, typo, correction)| typos::Typo {
                byte_offset,
                typo: typo.into(),
                corrections: typos::Status::Corrections(vec![correction.into()]),
            });
        let actual = fix_buffer(line, corrections);
        String::from_utf8(actual).unwrap()
    }

    #[test]
    fn test_fix_buffer_single() {
        let actual = fix_simple("foo foo foo", vec![(4, "foo", "bar")]);
        assert_eq!(actual, "foo bar foo");
    }

    #[test]
    fn test_fix_buffer_single_grow() {
        let actual = fix_simple("foo foo foo", vec![(4, "foo", "happy")]);
        assert_eq!(actual, "foo happy foo");
    }

    #[test]
    fn test_fix_buffer_single_shrink() {
        let actual = fix_simple("foo foo foo", vec![(4, "foo", "if")]);
        assert_eq!(actual, "foo if foo");
    }

    #[test]
    fn test_fix_buffer_start() {
        let actual = fix_simple("foo foo foo", vec![(0, "foo", "bar")]);
        assert_eq!(actual, "bar foo foo");
    }

    #[test]
    fn test_fix_buffer_end() {
        let actual = fix_simple("foo foo foo", vec![(8, "foo", "bar")]);
        assert_eq!(actual, "foo foo bar");
    }

    #[test]
    fn test_fix_buffer_end_grow() {
        let actual = fix_simple("foo foo foo", vec![(8, "foo", "happy")]);
        assert_eq!(actual, "foo foo happy");
    }

    #[test]
    fn test_fix_buffer_multiple() {
        let actual = fix_simple(
            "foo foo foo",
            vec![(4, "foo", "happy"), (8, "foo", "world")],
        );
        assert_eq!(actual, "foo happy world");
    }

    #[test]
    fn test_line_count_first() {
        let mut accum_line_num = AccumulateLineNum::new();
        let line_num = accum_line_num.line_num(b"hello world", 6);
        assert_eq!(line_num, 1);
    }

    #[test]
    fn test_line_count_second() {
        let mut accum_line_num = AccumulateLineNum::new();
        let line_num = accum_line_num.line_num(b"1\n2\n3", 2);
        assert_eq!(line_num, 2);
    }

    #[test]
    fn test_line_count_multiple() {
        let mut accum_line_num = AccumulateLineNum::new();
        let line_num = accum_line_num.line_num(b"1\n2\n3", 0);
        assert_eq!(line_num, 1);
        let line_num = accum_line_num.line_num(b"1\n2\n3", 2);
        assert_eq!(line_num, 2);
        let line_num = accum_line_num.line_num(b"1\n2\n3", 4);
        assert_eq!(line_num, 3);
    }

    #[test]
    fn test_extract_line_single_line() {
        let buffer = b"hello world";
        let buffer_offset = 6;
        let expected_line = b"hello world";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 6);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }

    #[test]
    fn test_extract_line_first() {
        let buffer = b"1\n2\n3";
        let buffer_offset = 0;
        let expected_line = b"1";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 0);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }

    #[test]
    fn test_extract_line_middle() {
        let buffer = b"1\n2\n3";
        let buffer_offset = 2;
        let expected_line = b"2";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 0);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }

    #[test]
    fn test_extract_line_end() {
        let buffer = b"1\n2\n3";
        let buffer_offset = 4;
        let expected_line = b"3";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 0);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }

    #[test]
    fn test_extract_line_offset_change() {
        let buffer = b"1\nhello world\n2";
        let buffer_offset = 8;
        let expected_line = b"hello world";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 6);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }

    #[test]
    fn test_extract_line_windows() {
        let buffer = b"1\r\nhello world\r\n2";
        let buffer_offset = 9;
        let expected_line = b"hello world";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 6);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }

    #[test]
    fn test_extract_line_slovak() {
        let buffer = b"LastErrorMessage=%1.%n%nChyba %2: %3\r\nSetupFileMissing=In\x9Atala\xE8n\xFD adres\xE1r neobsahuje s\xFAbor %1. Opravte, pros\xEDm, t\xFAto chybu alebo si zaobstarajte nov\xFA k\xF3piu tohto produktu.\r\nSetupFileCorrupt=S\xFAbory sprievodcu in\x9Atal\xE1ciou s\xFA po\x9Akoden\xE9. Zaobstarajte si, pros\xEDm, nov\xFA k\xF3piu tohto produktu.";
        let buffer_offset = 66;
        let expected_line = b"SetupFileMissing=In\x9Atala\xE8n\xFD adres\xE1r neobsahuje s\xFAbor %1. Opravte, pros\xEDm, t\xFAto chybu alebo si zaobstarajte nov\xFA k\xF3piu tohto produktu.";
        let (line, offset) = extract_line(buffer, buffer_offset);
        assert_eq!(line, expected_line);
        assert_eq!(offset, 28);
        assert_eq!(line[offset], buffer[buffer_offset]);
    }
}
