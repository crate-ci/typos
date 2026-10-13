#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[rustfmt::skip]
mod word_codegen;

pub use crate::word_codegen::WORD;
