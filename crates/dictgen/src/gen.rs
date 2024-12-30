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
    pub fn map(self) -> crate::DictMapGen<'g> {
        crate::DictMapGen { gen: self }
    }

    pub fn table(self) -> crate::DictTableGen<'g> {
        crate::DictTableGen { gen: self }
    }

    pub fn trie(self) -> crate::DictTrieGen<'g> {
        crate::DictTrieGen {
            gen: self,
            limit: 64,
        }
    }
}

impl Default for DictGen<'static> {
    fn default() -> Self {
        Self::new()
    }
}
