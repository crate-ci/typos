use bstr::ByteSlice;

use crate::report;
use crate::tokens;
use crate::Dictionary;

pub trait Check: Send + Sync {
    fn check_str(
        &self,
        buffer: &str,
        parser: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error>;

    fn check_bytes(
        &self,
        buffer: &[u8],
        parser: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error>;

    fn check_filenames(&self) -> bool;

    fn check_files(&self) -> bool;

    fn binary(&self) -> bool;

    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        if !self.check_filenames() {
            return Ok(());
        }

        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            let context_reporter = ReportContext {
                reporter,
                context: report::PathContext { path }.into(),
            };
            self.check_str(file_name, parser, dictionary, &context_reporter)?;
        }

        Ok(())
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        if !self.check_files() {
            return Ok(());
        }

        let buffer = read_file(path, reporter)?;
        let (buffer, content_type) = massage_data(buffer)?;
        if !explicit && !self.binary() && content_type.is_binary() {
            let msg = report::BinaryFile { path };
            reporter.report(msg.into())?;
            return Ok(());
        }

        for (line_idx, line) in buffer.lines().enumerate() {
            let line_num = line_idx + 1;
            let context_reporter = ReportContext {
                reporter,
                context: report::FileContext { path, line_num }.into(),
            };
            self.check_bytes(line, parser, dictionary, &context_reporter)?;
        }

        Ok(())
    }
}

struct ReportContext<'m, 'r> {
    reporter: &'r dyn report::Report,
    context: report::Context<'m>,
}

impl<'m, 'r> report::Report for ReportContext<'m, 'r> {
    fn report(&self, msg: report::Message) -> Result<(), std::io::Error> {
        let msg = msg.context(Some(self.context.clone()));
        self.reporter.report(msg)
    }
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

    pub fn build_identifier_parser(&self) -> ParseIdentifiers {
        ParseIdentifiers {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_word_parser(&self) -> ParseWords {
        ParseWords {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_files(&self) -> Files {
        Files {}
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
    fn check_str(
        &self,
        buffer: &str,
        tokenizer: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = crate::ParserBuilder::new()
            .tokenizer(tokenizer)
            .dictionary(dictionary)
            .typos();
        for typo in parser.parse_str(buffer) {
            let msg = report::Typo {
                context: None,
                buffer: std::borrow::Cow::Borrowed(buffer.as_bytes()),
                byte_offset: typo.byte_offset,
                typo: typo.typo,
                corrections: typo.corrections,
            };
            reporter.report(msg.into())?;
        }
        Ok(())
    }

    fn check_bytes(
        &self,
        buffer: &[u8],
        tokenizer: &tokens::Tokenizer,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = crate::ParserBuilder::new()
            .tokenizer(tokenizer)
            .dictionary(dictionary)
            .typos();
        for typo in parser.parse_bytes(buffer) {
            let msg = report::Typo {
                context: None,
                buffer: std::borrow::Cow::Borrowed(buffer.as_bytes()),
                byte_offset: typo.byte_offset,
                typo: typo.typo,
                corrections: typo.corrections,
            };
            reporter.report(msg.into())?;
        }
        Ok(())
    }

    fn check_filenames(&self) -> bool {
        self.check_filenames
    }

    fn check_files(&self) -> bool {
        self.check_files
    }

    fn binary(&self) -> bool {
        self.binary
    }
}

#[derive(Debug, Clone)]
pub struct ParseIdentifiers {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for ParseIdentifiers {
    fn check_str(
        &self,
        buffer: &str,
        tokenizer: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = crate::ParserBuilder::new()
            .tokenizer(tokenizer)
            .identifiers();
        for word in parser.parse_str(buffer) {
            let msg = report::Parse {
                context: None,
                kind: report::ParseKind::Word,
                data: word.token(),
            };
            reporter.report(msg.into())?;
        }

        Ok(())
    }

    fn check_bytes(
        &self,
        buffer: &[u8],
        tokenizer: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = crate::ParserBuilder::new()
            .tokenizer(tokenizer)
            .identifiers();
        for word in parser.parse_bytes(buffer) {
            let msg = report::Parse {
                context: None,
                kind: report::ParseKind::Word,
                data: word.token(),
            };
            reporter.report(msg.into())?;
        }

        Ok(())
    }

    fn check_filenames(&self) -> bool {
        self.check_filenames
    }

    fn check_files(&self) -> bool {
        self.check_files
    }

    fn binary(&self) -> bool {
        self.binary
    }
}

#[derive(Debug, Clone)]
pub struct ParseWords {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for ParseWords {
    fn check_str(
        &self,
        buffer: &str,
        tokenizer: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let word_parser = crate::ParserBuilder::new().tokenizer(tokenizer).words();
        for word in word_parser.parse_str(buffer) {
            let msg = report::Parse {
                context: None,
                kind: report::ParseKind::Word,
                data: word.token(),
            };
            reporter.report(msg.into())?;
        }

        Ok(())
    }

    fn check_bytes(
        &self,
        buffer: &[u8],
        tokenizer: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let parser = crate::ParserBuilder::new().tokenizer(tokenizer).words();
        for word in parser.parse_bytes(buffer) {
            let msg = report::Parse {
                context: None,
                kind: report::ParseKind::Word,
                data: word.token(),
            };
            reporter.report(msg.into())?;
        }

        Ok(())
    }

    fn check_filenames(&self) -> bool {
        self.check_filenames
    }

    fn check_files(&self) -> bool {
        self.check_files
    }

    fn binary(&self) -> bool {
        self.binary
    }
}

#[derive(Debug, Clone)]
pub struct Files {}

impl Check for Files {
    fn check_str(
        &self,
        _buffer: &str,
        _parser: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        _reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn check_bytes(
        &self,
        _buffer: &[u8],
        _parser: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        _reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn check_filenames(&self) -> bool {
        true
    }

    fn check_files(&self) -> bool {
        true
    }

    fn binary(&self) -> bool {
        true
    }

    fn check_filename(
        &self,
        _path: &std::path::Path,
        _parser: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        _reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        _explicit: bool,
        _parser: &tokens::Tokenizer,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<(), std::io::Error> {
        let msg = report::File::new(path);
        reporter.report(msg.into())?;

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
