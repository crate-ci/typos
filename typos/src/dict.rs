use std::borrow::Cow;

use unicase::UniCase;

use crate::tokens::Case;

pub trait Dictionary {
    fn correct_ident<'s, 'w>(
        &'s self,
        _ident: crate::tokens::Identifier<'w>,
    ) -> Option<Cow<'s, str>>;

    fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Option<Cow<'s, str>>;
}

#[derive(Default)]
pub struct BuiltIn {}

impl BuiltIn {
    pub fn new() -> Self {
        Self {}
    }

    pub fn correct_ident<'s, 'w>(
        &'s self,
        _ident: crate::tokens::Identifier<'w>,
    ) -> Option<Cow<'s, str>> {
        None
    }

    pub fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Option<Cow<'s, str>> {
        map_lookup(&crate::dict_codegen::WORD_DICTIONARY, word.token())
            .map(|s| case_correct(s, word.case()))
    }
}

impl Dictionary for BuiltIn {
    fn correct_ident<'s, 'w>(
        &'s self,
        ident: crate::tokens::Identifier<'w>,
    ) -> Option<Cow<'s, str>> {
        BuiltIn::correct_ident(self, ident)
    }

    fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Option<Cow<'s, str>> {
        BuiltIn::correct_word(self, word)
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

fn case_correct(correction: &str, case: Case) -> Cow<'_, str> {
    match case {
        Case::Lower | Case::None => correction.into(),
        Case::Title => {
            let mut title = String::with_capacity(correction.as_bytes().len());
            let mut char_indices = correction.char_indices();
            if let Some((_, c)) = char_indices.next() {
                title.extend(c.to_uppercase());
                if let Some((i, _)) = char_indices.next() {
                    title.push_str(&correction[i..]);
                }
            }
            title.into()
        }
        Case::Scream => correction
            .chars()
            .flat_map(|c| c.to_uppercase())
            .collect::<String>()
            .into(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_correct() {
        let cases = [
            ("foo", Case::Lower, "foo"),
            ("foo", Case::None, "foo"),
            ("foo", Case::Title, "Foo"),
            ("foo", Case::Scream, "FOO"),
            ("fOo", Case::None, "fOo"),
        ];
        for (correction, case, expected) in cases.iter() {
            let actual = case_correct(correction, *case);
            assert_eq!(*expected, actual);
        }
    }
}
