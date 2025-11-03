#[cfg(feature = "codegen")]
pub struct DictGen<'g> {
    pub(crate) name: &'g str,
    pub(crate) value_type: &'g str,
}

impl DictGen<'static> {
    pub fn new() -> Self {
        Self {
            name: "DICT",
            value_type: "&'static str",
        }
    }
}

impl<'g> DictGen<'g> {
    pub fn name<'n>(self, name: &'n str) -> DictGen<'n>
    where
        'g: 'n,
    {
        DictGen {
            name,
            value_type: self.value_type,
        }
    }

    pub fn value_type<'t>(self, value_type: &'t str) -> DictGen<'t>
    where
        'g: 't,
    {
        DictGen {
            name: self.name,
            value_type,
        }
    }

    #[cfg(feature = "map")]
    pub fn map(self) -> crate::MapGen<'g> {
        crate::MapGen {
            r#gen: self,
            unicode: true,
            unicase: true,
        }
    }

    pub fn ordered_map(self) -> crate::OrderedMapGen<'g> {
        crate::OrderedMapGen {
            r#gen: self,
            unicode: true,
            unicase: true,
        }
    }

    pub fn trie(self) -> crate::TrieGen<'g> {
        crate::TrieGen {
            r#gen: self,
            limit: 64,
        }
    }

    pub fn r#match(self) -> crate::MatchGen<'g> {
        crate::MatchGen { r#gen: self }
    }

    #[cfg(feature = "aho-corasick")]
    pub fn aho_corasick(self) -> crate::AhoCorasickGen<'g> {
        crate::AhoCorasickGen { r#gen: self }
    }
}

impl Default for DictGen<'static> {
    fn default() -> Self {
        Self::new()
    }
}
