mod parser;

pub use parser::ClusterIter;

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

fn imply(variants: &mut Vec<Variant>, required: Category, missing: Category) {
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Type {
    pub category: Category,
    pub tag: Option<Tag>,
    pub num: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Category {
    American,
    BritishIse,
    BritishIze,
    Canadian,
    Australian,
    Other,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Tag {
    Eq,
    Variant,
    Seldom,
    Possible,
    Improper,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Pos {
    Noun,
    Verb,
    Adjective,
    Adverb,
}
