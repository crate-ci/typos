mod codegen;

pub use codegen::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cluster {
    pub header: Option<&'static str>,
    pub entries: &'static [Entry],
    pub notes: &'static [&'static str],
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Entry {
    pub variants: &'static [Variant],
    pub pos: Option<Pos>,
    pub archaic: bool,
    pub note: bool,
    pub description: Option<&'static str>,
    pub comment: Option<&'static str>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Variant {
    pub types: &'static [Type],
    pub word: &'static str,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Type {
    pub category: Category,
    pub tag: Option<Tag>,
    pub num: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "flags", derive(enumflags2::BitFlags))]
#[repr(u8)]
pub enum Category {
    American = 0x01,
    BritishIse = 0x02,
    BritishIze = 0x04,
    Canadian = 0x08,
    Australian = 0x10,
    Other = 0x20,
}

#[cfg(feature = "flags")]
pub type CategorySet = enumflags2::BitFlags<Category>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "flags", derive(enumflags2::BitFlags))]
#[repr(u8)]
pub enum Tag {
    Eq = 0x01,
    Variant = 0x02,
    Seldom = 0x04,
    Possible = 0x08,
    Improper = 0x10,
}

#[cfg(feature = "flags")]
pub type TagSet = enumflags2::BitFlags<Tag>;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "flags", derive(enumflags2::BitFlags))]
#[repr(u8)]
pub enum Pos {
    Noun = 0x01,
    Verb = 0x02,
    Adjective = 0x04,
    Adverb = 0x08,
}

#[cfg(feature = "flags")]
pub type PosSet = enumflags2::BitFlags<Pos>;
