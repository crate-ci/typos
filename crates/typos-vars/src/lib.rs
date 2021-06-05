mod vars_codegen;

pub use crate::vars_codegen::*;

pub use varcon_core::Category;
pub use varcon_core::CategorySet;

pub fn find(word: &'_ unicase::UniCase<&str>) -> Option<&'static [(u8, &'static VariantsMap)]> {
    VARS_DICTIONARY
        .binary_search_by_key(word, |(key, _)| key.convert())
        .map(|i| VARS_DICTIONARY[i].1)
        .ok()
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum EncodedStr {
    //Unicode(&'static str),
    Ascii(&'static str),
}

impl EncodedStr {
    fn convert(self) -> unicase::UniCase<&'static str> {
        match self {
            //EncodedStr::Unicode(s) => unicase::UniCase::unicode(s),
            EncodedStr::Ascii(s) => unicase::UniCase::ascii(s),
        }
    }
}
