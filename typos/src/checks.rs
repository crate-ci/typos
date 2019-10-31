use bstr::ByteSlice;

use crate::report;
use crate::tokens;
use crate::Dictionary;

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

    pub fn build_checks<'d, 'p>(
        &self,
        dictionary: &'d dyn Dictionary,
        parser: &'p tokens::Parser,
    ) -> Checks<'d, 'p> {
        Checks {
            dictionary,
            parser,
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_identifier_parser<'p>(&self, parser: &'p tokens::Parser) -> ParseIdentifiers<'p> {
        ParseIdentifiers {
            parser,
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
        }
    }

    pub fn build_word_parser<'p>(&self, parser: &'p tokens::Parser) -> ParseWords<'p> {
        ParseWords {
            parser,
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

#[derive(Clone)]
pub struct ParseIdentifiers<'p> {
    parser: &'p tokens::Parser,
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl<'p> ParseIdentifiers<'p> {
    pub fn check_filename(
        &self,
        path: &std::path::Path,
        report: report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        if !self.check_filenames {
            return Ok(typos_found);
        }

        for part in path.components().filter_map(|c| c.as_os_str().to_str()) {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Identifier,
                data: self.parser.parse(part).map(|i| i.token()).collect(),
                non_exhaustive: (),
            };
            report(msg.into());
        }

        Ok(typos_found)
    }

    pub fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        report: report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        if !self.check_files {
            return Ok(typos_found);
        }

        let buffer = std::fs::read(path)
            .map_err(|e| crate::ErrorKind::IoError.into_error().with_source(e))?;
        if !explicit && !self.binary && is_binary(&buffer) {
            let msg = report::BinaryFile {
                path,
                non_exhaustive: (),
            };
            report(msg.into());
            return Ok(typos_found);
        }

        for line in buffer.lines() {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Identifier,
                data: self.parser.parse_bytes(line).map(|i| i.token()).collect(),
                non_exhaustive: (),
            };
            report(msg.into());
        }

        Ok(typos_found)
    }
}

impl std::fmt::Debug for ParseIdentifiers<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Checks")
            .field("parser", self.parser)
            .field("check_filenames", &self.check_filenames)
            .field("check_files", &self.check_files)
            .field("binary", &self.binary)
            .finish()
    }
}

#[derive(Clone)]
pub struct ParseWords<'p> {
    parser: &'p tokens::Parser,
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl<'p> ParseWords<'p> {
    pub fn check_filename(
        &self,
        path: &std::path::Path,
        report: report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        if !self.check_filenames {
            return Ok(typos_found);
        }

        for part in path.components().filter_map(|c| c.as_os_str().to_str()) {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Word,
                data: self
                    .parser
                    .parse(part)
                    .flat_map(|ident| ident.split().map(|i| i.token()))
                    .collect(),
                non_exhaustive: (),
            };
            report(msg.into());
        }

        Ok(typos_found)
    }

    pub fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        report: report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        if !self.check_files {
            return Ok(typos_found);
        }

        let buffer = std::fs::read(path)
            .map_err(|e| crate::ErrorKind::IoError.into_error().with_source(e))?;
        if !explicit && !self.binary && is_binary(&buffer) {
            let msg = report::BinaryFile {
                path,
                non_exhaustive: (),
            };
            report(msg.into());
            return Ok(typos_found);
        }

        for line in buffer.lines() {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Word,
                data: self
                    .parser
                    .parse_bytes(line)
                    .flat_map(|ident| ident.split().map(|i| i.token()))
                    .collect(),
                non_exhaustive: (),
            };
            report(msg.into());
        }

        Ok(typos_found)
    }
}

impl std::fmt::Debug for ParseWords<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Checks")
            .field("parser", self.parser)
            .field("check_filenames", &self.check_filenames)
            .field("check_files", &self.check_files)
            .field("binary", &self.binary)
            .finish()
    }
}

#[derive(Clone)]
pub struct Checks<'d, 'p> {
    dictionary: &'d dyn Dictionary,
    parser: &'p tokens::Parser,
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl<'d, 'p> Checks<'d, 'p> {
    pub fn check_filename(
        &self,
        path: &std::path::Path,
        report: report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        if !self.check_filenames {
            return Ok(typos_found);
        }

        for part in path.components().filter_map(|c| c.as_os_str().to_str()) {
            for ident in self.parser.parse(part) {
                if let Some(correction) = self.dictionary.correct_ident(ident) {
                    let msg = report::FilenameCorrection {
                        path,
                        typo: ident.token(),
                        correction,
                        non_exhaustive: (),
                    };
                    report(msg.into());
                    typos_found = true;
                } else {
                    for word in ident.split() {
                        if let Some(correction) = self.dictionary.correct_word(word) {
                            let msg = report::FilenameCorrection {
                                path,
                                typo: word.token(),
                                correction,
                                non_exhaustive: (),
                            };
                            report(msg.into());
                            typos_found = true;
                        }
                    }
                }
            }
        }

        Ok(typos_found)
    }

    pub fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        report: report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        if !self.check_files {
            return Ok(typos_found);
        }

        let buffer = std::fs::read(path)
            .map_err(|e| crate::ErrorKind::IoError.into_error().with_source(e))?;
        if !explicit && !self.binary && is_binary(&buffer) {
            let msg = report::BinaryFile {
                path,
                non_exhaustive: (),
            };
            report(msg.into());
            return Ok(typos_found);
        }

        for (line_idx, line) in buffer.lines().enumerate() {
            let line_num = line_idx + 1;
            for ident in self.parser.parse_bytes(line) {
                if let Some(correction) = self.dictionary.correct_ident(ident) {
                    let col_num = ident.offset();
                    let msg = report::Correction {
                        path,
                        line,
                        line_num,
                        col_num,
                        typo: ident.token(),
                        correction,
                        non_exhaustive: (),
                    };
                    typos_found = true;
                    report(msg.into());
                } else {
                    for word in ident.split() {
                        if let Some(correction) = self.dictionary.correct_word(word) {
                            let col_num = word.offset();
                            let msg = report::Correction {
                                path,
                                line,
                                line_num,
                                col_num,
                                typo: word.token(),
                                correction,
                                non_exhaustive: (),
                            };
                            typos_found = true;
                            report(msg.into());
                        }
                    }
                }
            }
        }

        Ok(typos_found)
    }
}

impl std::fmt::Debug for Checks<'_, '_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Checks")
            .field("parser", self.parser)
            .field("check_filenames", &self.check_filenames)
            .field("check_files", &self.check_files)
            .field("binary", &self.binary)
            .finish()
    }
}

fn is_binary(buffer: &[u8]) -> bool {
    let null_max = std::cmp::min(buffer.len(), 1024);
    buffer[0..null_max].find_byte(b'\0').is_some()
}
