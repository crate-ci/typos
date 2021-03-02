use bstr::ByteSlice;
use encoding::Encoding;
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
                for typo in typos::check_bytes(&buffer, policy.tokenizer, policy.dict) {
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
                for typo in typos::check_bytes(&buffer, policy.tokenizer, policy.dict) {
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
                for typo in typos::check_bytes(&buffer, policy.tokenizer, policy.dict) {
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
            for line in diff {
                print!("{}", line);
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
                for word in policy.tokenizer.parse_bytes(&buffer) {
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
                for word in policy
                    .tokenizer
                    .parse_bytes(&buffer)
                    .flat_map(|i| i.split())
                {
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

pub fn read_file(
    path: &std::path::Path,
    reporter: &dyn report::Report,
) -> Result<(Vec<u8>, content_inspector::ContentType), std::io::Error> {
    let buffer = if path == std::path::Path::new("-") {
        let mut buffer = Vec::new();
        report_result(std::io::stdin().read_to_end(&mut buffer), reporter)?;
        buffer
    } else {
        report_result(std::fs::read(path), reporter)?
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
            let buffer = report_result(encoding::all::UTF_16LE.decode(&buffer, encoding::DecoderTrap::Strict), reporter)?;
            (buffer.into_bytes(), content_type)
        }
        content_inspector::ContentType::UTF_16BE => {
            let buffer = report_result(encoding::all::UTF_16BE.decode(&buffer, encoding::DecoderTrap::Strict), reporter)?;
            (buffer.into_bytes(), content_type)
        },
    };

    Ok((buffer, content_type))
}

pub fn write_file(
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
            let buffer = report_result(String::from_utf8(buffer), reporter)?;
            if buffer.is_empty() {
                // Error occurred, don't clear out the file
                return Ok(());
            }
            report_result(
                encoding::all::UTF_16LE.encode(&buffer, encoding::EncoderTrap::Strict),
                reporter,
            )?
        }
        content_inspector::ContentType::UTF_16BE => {
            let buffer = report_result(String::from_utf8(buffer), reporter)?;
            if buffer.is_empty() {
                // Error occurred, don't clear out the file
                return Ok(());
            }
            report_result(
                encoding::all::UTF_16BE.encode(&buffer, encoding::EncoderTrap::Strict),
                reporter,
            )?
        }
    };

    if path == std::path::Path::new("-") {
        report_result(std::io::stdout().write_all(&buffer), reporter)?;
    } else {
        report_result(std::fs::write(path, buffer), reporter)?;
    }

    Ok(())
}

fn report_result<T: Default, E: ToString>(
    value: Result<T, E>,
    reporter: &dyn report::Report,
) -> Result<T, std::io::Error> {
    let buffer = match value {
        Ok(value) => value,
        Err(err) => {
            report_error(err, reporter)?;
            Default::default()
        }
    };
    Ok(buffer)
}

fn report_error<E: ToString>(err: E, reporter: &dyn report::Report) -> Result<(), std::io::Error> {
    let msg = report::Error::new(err.to_string());
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
        let newlines = slice.lines().count();
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
    policy: &crate::policy::Policy,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    for entry in walk {
        walk_entry(entry, checks, policy, reporter)?;
    }
    Ok(())
}

pub fn walk_path_parallel(
    walk: ignore::WalkParallel,
    checks: &dyn FileChecker,
    policy: &crate::policy::Policy,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    let error: std::sync::Mutex<Result<(), ignore::Error>> = std::sync::Mutex::new(Ok(()));
    walk.run(|| {
        Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
            match walk_entry(entry, checks, policy, reporter) {
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
    policy: &crate::policy::Policy,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(err) => {
            report_error(err, reporter)?;
            return Ok(());
        }
    };
    if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
        let explicit = entry.depth() == 0;
        let path = if entry.is_stdin() {
            std::path::Path::new("-")
        } else {
            entry.path()
        };
        checks.check_file(path, explicit, policy, reporter)?;
    }

    Ok(())
}
