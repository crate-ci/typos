#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "codegen")]
mod gen;
mod insensitive;
#[cfg(feature = "map")]
mod map;
mod ordered_map;
mod trie;

#[cfg(feature = "codegen")]
pub use gen::*;
pub use insensitive::*;
#[cfg(feature = "map")]
pub use map::*;
pub use ordered_map::*;
pub use trie::*;
