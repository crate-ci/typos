mod vars_codegen;

pub use crate::vars_codegen::*;

pub use varcon_core::Category;
pub use varcon_core::CategorySet;

pub fn find(word: &'_ unicase::UniCase<&str>) -> Option<&'static [(u8, &'static VariantsMap)]> {
    VARS_DICTIONARY
        .binary_search_by_key(word, |(key, _)| *key)
        .map(|i| VARS_DICTIONARY[i].1)
        .ok()
}
