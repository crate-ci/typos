pub mod borrowed;

#[cfg(feature = "parser")]
mod parser;

#[cfg(feature = "parser")]
pub use crate::parser::ClusterIter;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cluster {
    pub header: Option<String>,
    pub entries: Vec<Entry>,
    pub notes: Vec<String>,
}

impl Cluster {
    pub fn infer(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.infer();
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Entry {
    pub variants: Vec<Variant>,
    pub pos: Option<Pos>,
    pub archaic: bool,
    pub note: bool,
    pub description: Option<String>,
    pub comment: Option<String>,
}

impl Entry {
    pub fn infer(&mut self) {
        imply(
            &mut self.variants,
            Category::BritishIse,
            Category::BritishIze,
        );
        imply(&mut self.variants, Category::BritishIze, Category::Canadian);
        imply(
            &mut self.variants,
            Category::BritishIse,
            Category::Australian,
        );
    }
}

fn imply(variants: &mut [Variant], required: Category, missing: Category) {
    let missing_exists = variants
        .iter()
        .any(|v| v.types.iter().any(|t| t.category == missing));
    if missing_exists {
        return;
    }

    for variant in variants.iter_mut() {
        let types: Vec<_> = variant
            .types
            .iter()
            .filter(|t| t.category == required)
            .cloned()
            .map(|mut t| {
                t.category = missing;
                t
            })
            .collect();
        variant.types.extend(types);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Variant {
    pub types: Vec<Type>,
    pub word: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Type {
    pub category: Category,
    pub tag: Option<Tag>,
    pub num: Option<usize>,
}

#[cfg_attr(feature = "flags", enumflags2::bitflags)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

#[cfg_attr(feature = "flags", enumflags2::bitflags)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

#[cfg_attr(feature = "flags", enumflags2::bitflags)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Pos {
    Noun = 0x01,
    Verb = 0x02,
    Adjective = 0x04,
    Adverb = 0x08,
}

#[cfg(feature = "flags")]
pub type PosSet = enumflags2::BitFlags<Pos>;
