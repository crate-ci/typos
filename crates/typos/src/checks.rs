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
        if !explicit && !self.binary {
            let content_type = content_inspector::inspect(&buffer);
            if content_type.is_binary()
                // HACK: We only support UTF-8 at the moment
                || (content_type != content_inspector::ContentType::UTF_8_BOM
                    && content_type != content_inspector::ContentType::UTF_8)
            {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into());
                return Ok(typos_found);
            }
        }

        for line in buffer.lines() {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Identifier,
                data: parser.parse_bytes(line).map(|i| i.token()).collect(),
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
        if !explicit && !self.binary {
            let content_type = content_inspector::inspect(&buffer);
            // HACK: We only support UTF-8 at the moment
            if content_type.is_binary()
                || (content_type != content_inspector::ContentType::UTF_8_BOM
                    && content_type != content_inspector::ContentType::UTF_8)
            {
                let msg = report::BinaryFile { path };
                reporter.report(msg.into());
                return Ok(typos_found);
            }
        }

        for line in buffer.lines() {
            let msg = report::Parse {
                path,
                kind: report::ParseKind::Word,
                data: parser
                    .parse_bytes(line)
                    .flat_map(|ident| ident.split().map(|i| i.token()))
                    .collect(),
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
                let corrections = dictionary.correct_ident(ident);
                if !corrections.is_empty() {
                    let byte_offset = ident.offset();
                    let msg = report::PathCorrection {
                        path,
                        byte_offset,
                        typo: ident.token(),
                        corrections,
                    };
                    typos_found |= reporter.report(msg.into());
                } else {
                    for word in ident.split() {
                        let corrections = dictionary.correct_word(word);
                        if !corrections.is_empty() {
                            let byte_offset = word.offset();
                            let msg = report::PathCorrection {
                                path,
                                byte_offset,
                                typo: word.token(),
                                corrections,
                            };
                            typos_found |= reporter.report(msg.into());
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
        if !explicit && !self.binary {
            let content_type = content_inspector::inspect(&buffer);
            // HACK: We only support UTF-8 at the moment
            if content_type.is_binary()
                || (content_type != content_inspector::ContentType::UTF_8_BOM
                    && content_type != content_inspector::ContentType::UTF_8)
            {
                // HACK: we don't support alternative encodings atm
                let msg = report::BinaryFile { path };
                reporter.report(msg.into());
                return Ok(typos_found);
            }
        }

        for (line_idx, line) in buffer.lines().enumerate() {
            let line_num = line_idx + 1;
            for ident in parser.parse_bytes(line) {
                let corrections = dictionary.correct_ident(ident);
                if !corrections.is_empty() {
                    let byte_offset = ident.offset();
                    let msg = report::Correction {
                        path,
                        line,
                        line_num,
                        byte_offset,
                        typo: ident.token(),
                        corrections,
                    };
                    typos_found |= reporter.report(msg.into());
                } else {
                    for word in ident.split() {
                        let corrections = dictionary.correct_word(word);
                        if !corrections.is_empty() {
                            let byte_offset = word.offset();
                            let msg = report::Correction {
                                path,
                                line,
                                line_num,
                                byte_offset,
                                typo: word.token(),
                                corrections,
                            };
                            typos_found |= reporter.report(msg.into());
                        }
                    }
                }
            }
        }

        Ok(typos_found)
    }
}
