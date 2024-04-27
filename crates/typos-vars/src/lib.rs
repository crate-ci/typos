#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod vars_codegen;

pub use crate::vars_codegen::*;

pub use varcon_core::Category;
pub use varcon_core::CategorySet;
