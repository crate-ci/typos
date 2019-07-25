#[macro_use]
extern crate serde_derive;

mod dict;
mod dict_codegen;

pub mod report;
pub mod tokens;

pub use crate::dict::*;

use std::fs::File;
use std::io::Read;

use bstr::ByteSlice;

pub fn process_file(
    path: &std::path::Path,
    dictionary: &Dictionary,
    check_filenames: bool,
    check_files: bool,
    parser: &tokens::Parser,
    binary: bool,
    report: report::Report,
) -> Result<bool, failure::Error> {
    let mut typos_found = false;

    if check_filenames {
        for part in path.components().filter_map(|c| c.as_os_str().to_str()) {
            for ident in parser.parse(part) {
                if let Some(correction) = dictionary.correct_ident(ident) {
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
                    if let Some(correction) = dictionary.correct_word(word) {
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

    if check_files {
        let mut buffer = Vec::new();
        File::open(path)?.read_to_end(&mut buffer)?;
        if !binary && buffer.find_byte(b'\0').is_some() {
            let msg = report::BinaryFile {
                path,
                non_exhaustive: (),
            };
            report(msg.into());
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
                    report(msg.into());
                }
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
                        report(msg.into());
                    }
                }
            }
        }
    }

    Ok(typos_found)
}
