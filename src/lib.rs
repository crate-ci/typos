#[macro_use]
extern crate serde_derive;

mod dict;
mod dict_codegen;

pub mod report;
pub mod tokens;

pub use crate::dict::*;

use std::fs::File;
use std::io::Read;

pub fn process_file(
    path: &std::path::Path,
    dictionary: &Dictionary,
    report: report::Report,
) -> Result<(), failure::Error> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    for (line_idx, line) in grep_searcher::LineIter::new(b'\n', &buffer).enumerate() {
        let line_num = line_idx + 1;
        for ident in tokens::Identifier::parse(line) {
            if let Some(correction) = dictionary.correct_ident(ident) {
                let col_num = ident.offset();
                let msg = report::Message {
                    path,
                    line,
                    line_num,
                    col_num,
                    word: ident.token(),
                    correction,
                    non_exhaustive: (),
                };
                report(msg);
            }
            for word in ident.split() {
                if let Some(correction) = dictionary.correct_word(word) {
                    let col_num = word.offset();
                    let msg = report::Message {
                        path,
                        line,
                        line_num,
                        col_num,
                        word: word.token(),
                        correction,
                        non_exhaustive: (),
                    };
                    report(msg);
                }
            }
        }
    }

    Ok(())
}
