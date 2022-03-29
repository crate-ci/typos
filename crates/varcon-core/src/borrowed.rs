#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Cluster {
    pub header: Option<&'static str>,
    pub entries: &'static [Entry],
    pub notes: &'static [&'static str],
}

impl Cluster {
    pub fn into_owned(self) -> crate::Cluster {
        crate::Cluster {
            header: self.header.map(|s| s.to_owned()),
            entries: self.entries.iter().map(|s| s.into_owned()).collect(),
            notes: self.notes.iter().map(|s| (*s).to_owned()).collect(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Entry {
    pub variants: &'static [Variant],
    pub pos: Option<crate::Pos>,
    pub archaic: bool,
    pub note: bool,
    pub description: Option<&'static str>,
    pub comment: Option<&'static str>,
}

impl Entry {
    pub fn into_owned(self) -> crate::Entry {
        crate::Entry {
            variants: self.variants.iter().map(|v| v.into_owned()).collect(),
            pos: self.pos,
            archaic: self.archaic,
            note: self.note,
            description: self.description.map(|s| s.to_owned()),
            comment: self.comment.map(|s| s.to_owned()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Variant {
    pub types: &'static [crate::Type],
    pub word: &'static str,
}

impl Variant {
    pub fn into_owned(self) -> crate::Variant {
        crate::Variant {
            types: self.types.to_vec(),
            word: self.word.to_owned(),
        }
    }
}
