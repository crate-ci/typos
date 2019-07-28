#[macro_use]
extern crate serde_derive;

mod dict;
mod dict_codegen;

pub mod checks;
pub mod report;
pub mod tokens;

pub use crate::dict::*;
