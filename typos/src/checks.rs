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

    pub fn build_checks(&self) -> Checks {
        Checks {
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

impl ParseIdentifiers {
    pub fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &tokens::Parser,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        if !self.check_filenames {
            return Ok(typos_found);
        }

        for part in path.components().filter_map(|c| c.as_os_str().to_str()) {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Identifier,
                data: parser.parse(part).map(|i| i.token()).collect(),
                non_exhaustive: (),
            };
            reporter.report(msg.into());
        }

        Ok(typos_found)
    }

    pub fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &tokens::Parser,
        reporter: &dyn report::Report,
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
            reporter.report(msg.into());
            return Ok(typos_found);
        }

        for line in buffer.lines() {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Identifier,
                data: parser.parse_bytes(line).map(|i| i.token()).collect(),
                non_exhaustive: (),
            };
            reporter.report(msg.into());
        }

        Ok(typos_found)
    }
}

#[derive(Debug, Clone)]
pub struct ParseWords {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl ParseWords {
    pub fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &tokens::Parser,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let typos_found = false;

        if !self.check_filenames {
            return Ok(typos_found);
        }

        for part in path.components().filter_map(|c| c.as_os_str().to_str()) {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Word,
                data: parser
                    .parse(part)
                    .flat_map(|ident| ident.split().map(|i| i.token()))
                    .collect(),
                non_exhaustive: (),
            };
            reporter.report(msg.into());
        }

        Ok(typos_found)
    }

    pub fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &tokens::Parser,
        reporter: &dyn report::Report,
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
            reporter.report(msg.into());
            return Ok(typos_found);
        }

        for line in buffer.lines() {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Word,
                data: parser
                    .parse_bytes(line)
                    .flat_map(|ident| ident.split().map(|i| i.token()))
                    .collect(),
                non_exhaustive: (),
            };
            reporter.report(msg.into());
        }

        Ok(typos_found)
    }
}

#[derive(Debug, Clone)]
pub struct Checks {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl Checks {
    pub fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
    ) -> Result<bool, crate::Error> {
        let mut typos_found = false;

        if !self.check_filenames {
            return Ok(typos_found);
        }

        if let Some(part) = path.file_name().and_then(|s| s.to_str()) {
            for ident in parser.parse(part) {
                if let Some(correction) = dictionary.correct_ident(ident) {
                    let msg = report::FilenameCorrection {
                        path,
                        typo: ident.token(),
                        correction,
                        non_exhaustive: (),
                    };
                    reporter.report(msg.into());
                    typos_found = true;
                } else {
                    for word in ident.split() {
                        if let Some(correction) = dictionary.correct_word(word) {
                            let msg = report::FilenameCorrection {
                                path,
                                typo: word.token(),
                                correction,
                                non_exhaustive: (),
                            };
                            reporter.report(msg.into());
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
        parser: &tokens::Parser,
        dictionary: &dyn Dictionary,
        reporter: &dyn report::Report,
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
            reporter.report(msg.into());
            return Ok(typos_found);
        }

        for (line_idx, line) in buffer.lines().enumerate() {
            let line_num = line_idx + 1;
            for ident in parser.parse_bytes(line) {
                if let Some(correction) = dictionary.correct_ident(ident) {
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
                    reporter.report(msg.into());
                } else {
                    for word in ident.split() {
                        if let Some(correction) = dictionary.correct_word(word) {
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
                            reporter.report(msg.into());
                        }
                    }
                }
            }
        }

        Ok(typos_found)
    }
}

fn is_binary(buffer: &[u8]) -> bool {
    let null_max = std::cmp::min(buffer.len(), 1024);
    buffer[0..null_max].find_byte(b'\0').is_some()
}
