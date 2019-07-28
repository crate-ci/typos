use std::fs::File;
use std::io::Read;

use bstr::ByteSlice;

use crate::report;
use crate::tokens;
use crate::Dictionary;

pub struct CheckSettings {
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

impl CheckSettings {
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

    pub fn build<'d, 'p>(
        &self,
        dictionary: &'d Dictionary,
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
}

impl Default for CheckSettings {
    fn default() -> Self {
        Self {
            check_filenames: true,
            check_files: true,
            binary: false,
        }
    }
}

pub struct Checks<'d, 'p> {
    dictionary: &'d Dictionary,
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
    ) -> Result<bool, failure::Error> {
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
                }
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

        Ok(typos_found)
    }

    pub fn check_file(
        &self,
        path: &std::path::Path,
        report: report::Report,
    ) -> Result<bool, failure::Error> {
        let mut typos_found = false;

        if !self.check_files {
            return Ok(typos_found);
        }

        let mut buffer = Vec::new();
        File::open(path)?.read_to_end(&mut buffer)?;
        if !self.binary && buffer.find_byte(b'\0').is_some() {
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
                }
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

        Ok(typos_found)
    }
}
