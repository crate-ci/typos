//! `typos_cli`'s API is unstable.  Open an issue for starting a discussion on getting a subset
//! stabilized.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[doc(hidden)]
pub mod config;
#[doc(hidden)]
pub mod dict;
#[doc(hidden)]
pub mod file;
#[doc(hidden)]
pub mod policy;
#[doc(hidden)]
pub mod report;

mod default_types;
mod file_type;
mod file_type_specifics;
