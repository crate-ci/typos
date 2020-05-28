mod codegen;

pub use codegen::*;
pub use varcon_core::borrowed::Cluster;
pub use varcon_core::borrowed::Entry;
pub use varcon_core::borrowed::Variant;
pub use varcon_core::Category;
#[cfg(feature = "flags")]
pub use varcon_core::CategorySet;
pub use varcon_core::Pos;
#[cfg(feature = "flags")]
pub use varcon_core::PosSet;
pub use varcon_core::Tag;
#[cfg(feature = "flags")]
pub use varcon_core::TagSet;
pub use varcon_core::Type;
