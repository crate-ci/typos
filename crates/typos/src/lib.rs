#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod check;
mod dict;

pub mod tokens;

pub use check::*;
pub use dict::*;
