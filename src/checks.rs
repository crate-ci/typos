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
                        typo: typo.typo,
                        corrections: typo.corrections,
                    };
                    reporter.report(msg.into())?;
                }
            }
        }

        if self.check_files {
            let buffer = read_file(path, reporter)?;
            let (buffer, content_type) = massage_data(buffer)?;
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
                        typo: typo.typo,
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
            let buffer = read_file(path, reporter)?;
            let (buffer, content_type) = massage_data(buffer)?;
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
            let buffer = read_file(path, reporter)?;
            let (buffer, content_type) = massage_data(buffer)?;
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
            let buffer = read_file(path, reporter)?;
            let (_buffer, content_type) = massage_data(buffer)?;
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
) -> Result<Vec<u8>, std::io::Error> {
    let buffer = match std::fs::read(path) {
        Ok(buffer) => buffer,
        Err(err) => {
            let msg = report::Error::new(err.to_string());
            reporter.report(msg.into())?;
            Vec::new()
        }
    };
    Ok(buffer)
}

fn massage_data(
    buffer: Vec<u8>,
) -> Result<(Vec<u8>, content_inspector::ContentType), std::io::Error> {
    let mut content_type = content_inspector::inspect(&buffer);

    // HACK: We only support UTF-8 at the moment
    if content_type != content_inspector::ContentType::UTF_8_BOM
        && content_type != content_inspector::ContentType::UTF_8
    {
        content_type = content_inspector::ContentType::BINARY;
    }

    Ok((buffer, content_type))
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
