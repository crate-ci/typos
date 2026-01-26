#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "aho-corasick")]
pub mod aho_corasick;
#[cfg(feature = "codegen")]
mod r#gen;
mod insensitive;
#[cfg(feature = "map")]
mod map;
#[cfg(feature = "codegen")]
mod r#match;
mod ordered_map;
mod trie;

#[cfg(feature = "aho-corasick")]
#[cfg(feature = "codegen")]
pub use aho_corasick::AhoCorasickGen;
#[cfg(feature = "codegen")]
pub use r#gen::*;
pub use insensitive::*;
#[cfg(feature = "map")]
pub use map::*;
#[cfg(feature = "codegen")]
pub use r#match::*;
pub use ordered_map::*;
pub use trie::*;
