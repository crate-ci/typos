use bstr::ByteSlice;

use crate::report;
use typos::tokens;
use typos::Dictionary;

pub trait Check: Send + Sync {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyposSettings {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl TyposSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn check_filenames(&mut self, yes: bool) -> &mut Self {
        self.check_filenames = yes;
        self
    }

    pub fn check_files(&mut self, yes: bool) -> &mut Self {
        self.check_files = yes;
        self
    }

    pub fn binary(&mut self, yes: bool) -> &mut Self {
        self.binary = yes;
        self
    }

    pub fn build_typos(&self) -> Typos {
        Typos {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_fix_typos(&self) -> FixTypos {
        FixTypos {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_diff_typos(&self) -> DiffTypos {
        DiffTypos {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_identifier_parser(&self) -> Identifiers {
        Identifiers {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_word_parser(&self) -> Words {
        Words {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_files(&self) -> FoundFiles {
        FoundFiles {
            binary: self.binary,
        }
    }
}

impl Default for TyposSettings {
    fn default() -> Self {
        Self {
            check_filenames: true,
            check_files: true,
            binary: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Typos {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for Typos {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        tokenizer: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = typos::ParserBuilder::new()
            .tokenizer(tokenizer)
            .dictionary(dictionary)
            .typos();

        if self.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                for typo in parser.parse_str(file_name) {
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

        if self.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !self.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut accum_line_num = AccumulateLineNum::new();
                for typo in parser.parse_bytes(&buffer) {
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

#[derive(Debug, Clone)]
pub struct FixTypos {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for FixTypos {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        tokenizer: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = typos::ParserBuilder::new()
            .tokenizer(tokenizer)
            .dictionary(dictionary)
            .typos();

        if self.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !self.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut fixes = Vec::new();
                let mut accum_line_num = AccumulateLineNum::new();
                for typo in parser.parse_bytes(&buffer) {
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
                    let buffer = fix_buffer(buffer, fixes.into_iter());
                    write_file(path, content_type, &buffer, reporter)?;
                }
            }
        }

        // Ensure the above write can happen before renaming the file.
        if self.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                let mut fixes = Vec::new();
                for typo in parser.parse_str(file_name) {
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

#[derive(Debug, Clone)]
pub struct DiffTypos {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for DiffTypos {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        tokenizer: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = typos::ParserBuilder::new()
            .tokenizer(tokenizer)
            .dictionary(dictionary)
            .typos();

        let mut content = Vec::new();
        let mut new_content = Vec::new();
        if self.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !self.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                let mut fixes = Vec::new();
                let mut accum_line_num = AccumulateLineNum::new();
                for typo in parser.parse_bytes(&buffer) {
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
        if self.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                let mut fixes = Vec::new();
                for typo in parser.parse_str(file_name) {
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
            let fixed_path = new_path
                .as_ref()
                .map(|p| p.as_path())
                .unwrap_or(path)
                .display()
                .to_string();
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

#[derive(Debug, Clone)]
pub struct Identifiers {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for Identifiers {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        tokenizer: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = typos::ParserBuilder::new()
            .tokenizer(tokenizer)
            .identifiers();

        if self.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                for word in parser.parse_str(file_name) {
                    let msg = report::Parse {
                        context: Some(report::PathContext { path }.into()),
                        kind: report::ParseKind::Identifier,
                        data: word.token(),
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        if self.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !self.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                for word in parser.parse_bytes(&buffer) {
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

#[derive(Debug, Clone)]
pub struct Words {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for Words {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        tokenizer: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = typos::ParserBuilder::new().tokenizer(tokenizer).words();

        if self.check_filenames {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                for word in parser.parse_str(file_name) {
                    let msg = report::Parse {
                        context: Some(report::PathContext { path }.into()),
                        kind: report::ParseKind::Word,
                        data: word.token(),
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        if self.check_files {
            let (buffer, content_type) = read_file(path, reporter)?;
            if !explicit && !self.binary && content_type.is_binary() {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into())?;
            } else {
                for word in parser.parse_bytes(&buffer) {
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

#[derive(Debug, Clone)]
pub struct FoundFiles {
    binary: bool,
}

impl Check for FoundFiles {
    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        _parser: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        // Check `self.binary` first so we can easily check performance of walking vs reading
        if self.binary {
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
    let buffer = match std::fs::read(path) {
        Ok(buffer) => buffer,
        Err(err) => {
            let msg = report::Error::new(err.to_string());
            reporter.report(msg.into())?;
            Vec::new()
        }
    };

    let mut content_type = content_inspector::inspect(&buffer);
    // HACK: We only support UTF-8 at the moment
    if content_type != content_inspector::ContentType::UTF_8_BOM
        && content_type != content_inspector::ContentType::UTF_8
    {
        content_type = content_inspector::ContentType::BINARY;
    }

    Ok((buffer, content_type))
}

pub fn write_file(
    path: &std::path::Path,
    content_type: content_inspector::ContentType,
    buffer: &[u8],
    reporter: &dyn report::Report,
) -> Result<(), std::io::Error> {
    assert!(
        content_type == content_inspector::ContentType::UTF_8_BOM
            || content_type == content_inspector::ContentType::UTF_8
            || content_type == content_inspector::ContentType::BINARY
    );
    match std::fs::write(path, buffer) {
        Ok(()) => (),
        Err(err) => {
            let msg = report::Error::new(err.to_string());
            reporter.report(msg.into())?;
        }
    };
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

fn is_fixable<'t>(typo: &typos::Typo<'t>) -> bool {
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

pub fn check_path(
    walk: ignore::Walk,
    checks: &dyn Check,
    parser: &typos::tokens::Tokenizer,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    for entry in walk {
        check_entry(entry, checks, parser, dictionary, reporter)?;
    }
    Ok(())
}

pub fn check_path_parallel(
    walk: ignore::WalkParallel,
    checks: &dyn Check,
    parser: &typos::tokens::Tokenizer,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    let error: std::sync::Mutex<Result<(), ignore::Error>> = std::sync::Mutex::new(Ok(()));
    walk.run(|| {
        Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
            match check_entry(entry, checks, parser, dictionary, reporter) {
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

fn check_entry(
    entry: Result<ignore::DirEntry, ignore::Error>,
    checks: &dyn Check,
    parser: &typos::tokens::Tokenizer,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn report::Report,
) -> Result<(), ignore::Error> {
    let entry = entry?;
    if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
        let explicit = entry.depth() == 0;
        checks.check_file(entry.path(), explicit, parser, dictionary, reporter)?;
    }

    Ok(())
}
