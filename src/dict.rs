use std::borrow::Cow;

use unicase::UniCase;

#[derive(Default)]
pub struct Dictionary {}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {}
    }

    pub fn correct_ident<'s, 'w>(
        &'s self,
        _ident: crate::tokens::Identifier<'w>,
    ) -> Option<Cow<'s, str>> {
        None
    }

    pub fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Option<Cow<'s, str>> {
        map_lookup(&crate::dict_codegen::WORD_DICTIONARY, word.token()).map(|s| s.into())
    }
}

fn map_lookup(
    map: &'static phf::Map<UniCase<&'static str>, &'static str>,
    key: &str,
) -> Option<&'static str> {
    // This transmute should be safe as `get` will not store the reference with
    // the expanded lifetime. This is due to `Borrow` being overly strict and
    // can't have an impl for `&'static str` to `Borrow<&'a str>`.
    //
    //
    // See https://github.com/rust-lang/rust/issues/28853#issuecomment-158735548
    unsafe {
        let key = ::std::mem::transmute::<_, &'static str>(key);
        map.get(&UniCase(key)).cloned()
    }
}
