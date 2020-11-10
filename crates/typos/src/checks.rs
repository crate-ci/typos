use bstr::ByteSlice;

use crate::report;
use crate::tokens;
use crate::Dictionary;
use crate::Status;

pub trait Check: Send + Sync {
    fn check_str(
        &self,
        buffer: &str,
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error>;

    fn check_bytes(
        &self,
        buffer: &[u8],
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error>;

    fn check_filenames(&self) -> bool;

    fn check_files(&self) -> bool;

    fn binary(&self) -> bool;

    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        if !self.check_filenames() {
            return Ok(typos_found);
        }

        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            let context_reporter = ReportContext {
                reporter,
                context: report::PathContext { path }.into(),
            };
            typos_found |= self.check_str(file_name, parser, dictionary, &context_reporter)?;
        }

        Ok(typos_found)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        if !self.check_files() {
            return Ok(typos_found);
        }

        let buffer = read_file(path)?;
        let (buffer, content_type) = massage_data(buffer)?;
        if !explicit && !self.binary() && content_type.is_binary() {
            let msg = report::BinaryFile { path };
            reporter.report(msg.into());
            return Ok(typos_found);
        }

        for (line_idx, line) in buffer.lines().enumerate() {
            let line_num = line_idx + 1;
            let context_reporter = ReportContext {
                reporter,
                context: report::FileContext { path, line_num }.into(),
            };
            typos_found |= self.check_bytes(line, parser, dictionary, &context_reporter)?;
        }

        Ok(typos_found)
    }
}

struct ReportContext<'m, 'r> {
    reporter: &'r dyn report::Report,
    context: report::Context<'m>,
}

impl<'m, 'r> report::Report for ReportContext<'m, 'r> {
    fn report(&self, msg: report::Message) -> bool {
        let msg = msg.context(self.context.clone());
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
pub struct ParseIdentifiers {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for ParseIdentifiers {
    fn check_str(
        &self,
        buffer: &str,
        parser: &tokens::Parser,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        let msg = report::Parse {
            context: report::Context::None,
            kind: report::ParseKind::Identifier,
            data: parser.parse_str(buffer).map(|i| i.token()).collect(),
        };
        reporter.report(msg.into());

        Ok(typos_found)
    }

    fn check_bytes(
        &self,
        buffer: &[u8],
        parser: &tokens::Parser,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        let msg = report::Parse {
            context: report::Context::None,
            kind: report::ParseKind::Identifier,
            data: parser.parse_bytes(buffer).map(|i| i.token()).collect(),
        };
        reporter.report(msg.into());

        Ok(typos_found)
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
        parser: &tokens::Parser,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        let msg = report::Parse {
            context: report::Context::None,
            kind: report::ParseKind::Word,
            data: parser
                .parse_str(buffer)
                .flat_map(|ident| ident.split().map(|i| i.token()))
                .collect(),
        };
        reporter.report(msg.into());

        Ok(typos_found)
    }

    fn check_bytes(
        &self,
        buffer: &[u8],
        parser: &tokens::Parser,
        _dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        let msg = report::Parse {
            context: report::Context::None,
            kind: report::ParseKind::Word,
            data: parser
                .parse_bytes(buffer)
                .flat_map(|ident| ident.split().map(|i| i.token()))
                .collect(),
        };
        reporter.report(msg.into());

        Ok(typos_found)
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
pub struct Typos {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Check for Typos {
    fn check_str(
        &self,
        buffer: &str,
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        for ident in parser.parse_str(buffer) {
            match dictionary.correct_ident(ident) {
                Some(Status::Valid) => {}
                Some(corrections) => {
                    let byte_offset = ident.offset();
                    let msg = report::Typo {
                        context: report::Context::None,
                        buffer: std::borrow::Cow::Borrowed(buffer.as_bytes()),
                        byte_offset,
                        typo: ident.token(),
                        corrections,
                    };
                    typos_found |= reporter.report(msg.into());
                }
                None => {
                    for word in ident.split() {
                        match dictionary.correct_word(word) {
                            Some(Status::Valid) => {}
                            Some(corrections) => {
                                let byte_offset = word.offset();
                                let msg = report::Typo {
                                    context: report::Context::None,
                                    buffer: std::borrow::Cow::Borrowed(buffer.as_bytes()),
                                    byte_offset,
                                    typo: word.token(),
                                    corrections,
                                };
                                typos_found |= reporter.report(msg.into());
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        Ok(typos_found)
    }

    fn check_bytes(
        &self,
        buffer: &[u8],
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        for ident in parser.parse_bytes(buffer) {
            match dictionary.correct_ident(ident) {
                Some(Status::Valid) => {}
                Some(corrections) => {
                    let byte_offset = ident.offset();
                    let msg = report::Typo {
                        context: report::Context::None,
                        buffer: std::borrow::Cow::Borrowed(buffer),
                        byte_offset,
                        typo: ident.token(),
                        corrections,
                    };
                    typos_found |= reporter.report(msg.into());
                }
                None => {
                    for word in ident.split() {
                        match dictionary.correct_word(word) {
                            Some(Status::Valid) => {}
                            Some(corrections) => {
                                let byte_offset = word.offset();
                                let msg = report::Typo {
                                    context: report::Context::None,
                                    buffer: std::borrow::Cow::Borrowed(buffer),
                                    byte_offset,
                                    typo: word.token(),
                                    corrections,
                                };
                                typos_found |= reporter.report(msg.into());
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        Ok(typos_found)
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

fn read_file(path: &std::path::Path) -> Result<Vec<u8>, crate::Error> {
    std::fs::read(path).map_err(|e| crate::ErrorKind::IoError.into_error().with_source(e))
}

fn massage_data(
    buffer: Vec<u8>,
) -> Result<(Vec<u8>, content_inspector::ContentType), crate::Error> {
    let mut content_type = content_inspector::inspect(&buffer);

    // HACK: We only support UTF-8 at the moment
    if content_type != content_inspector::ContentType::UTF_8_BOM
        && content_type != content_inspector::ContentType::UTF_8
    {
        content_type = content_inspector::ContentType::BINARY;
    }

    Ok((buffer, content_type))
}
