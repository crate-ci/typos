use std::borrow::Cow;
use std::collections::HashSet;

use unicase::UniCase;

use typos::tokens::Case;

#[derive(Default)]
pub struct BuiltIn {
    locale: Option<typos_vars::Category>,
}

impl BuiltIn {
    pub fn new(locale: crate::config::Locale) -> Self {
        Self {
            locale: locale.category(),
        }
    }

    pub fn correct_ident<'s, 'w>(
        &'s self,
        _ident: typos::tokens::Identifier<'w>,
    ) -> Vec<Cow<'s, str>> {
        Vec::new()
    }

    pub fn correct_word<'s, 'w>(
        &'s self,
        word_token: typos::tokens::Word<'w>,
    ) -> Vec<Cow<'s, str>> {
        let word = word_token.token();
        let corrections = if let Some(correction) = self.correct_with_dict(word) {
            self.correct_with_vars(word)
                .unwrap_or_else(|| vec![correction])
        } else {
            self.correct_with_vars(word).unwrap_or_else(Vec::new)
        };
        corrections
            .into_iter()
            .map(|s| case_correct(s, word_token.case()))
            .collect()
    }

    fn correct_with_dict(&self, word: &str) -> Option<&'static str> {
        map_lookup(&typos_dict::WORD_DICTIONARY, word)
    }

    fn correct_with_vars(&self, word: &str) -> Option<Vec<&'static str>> {
        let variants = map_lookup(&typos_vars::VARS_DICTIONARY, word)?;
        self.select_variant(variants)
    }

    fn select_variant(
        &self,
        vars: &'static [(u8, &'static typos_vars::VariantsMap)],
    ) -> Option<Vec<&'static str>> {
        let var = vars[0];
        let var_categories = unsafe {
            // Code-genned from a checked category-set, so known to be safe
            typos_vars::CategorySet::new(var.0)
        };
        if let Some(locale) = self.locale {
            if var_categories.contains(locale) {
                // Already valid for the current locale.
                None
            } else {
                Some(
                    typos_vars::corrections(locale, *var.1)
                        .iter()
                        .copied()
                        .collect(),
                )
            }
        } else {
            // All locales are valid
            if var_categories.is_empty() {
                // But the word is never valid.
                let mut unique: Vec<_> = var.1.iter().flat_map(|v| v.iter()).copied().collect();
                unique.sort_unstable();
                unique.dedup();
                Some(unique)
            } else {
                None
            }
        }
    }
}

impl typos::Dictionary for BuiltIn {
    fn correct_ident<'s, 'w>(&'s self, ident: typos::tokens::Identifier<'w>) -> Vec<Cow<'s, str>> {
        BuiltIn::correct_ident(self, ident)
    }

    fn correct_word<'s, 'w>(&'s self, word: typos::tokens::Word<'w>) -> Vec<Cow<'s, str>> {
        BuiltIn::correct_word(self, word)
    }
}

fn map_lookup<V: Clone>(map: &'static phf::Map<UniCase<&'static str>, V>, key: &str) -> Option<V> {
    // This transmute should be safe as `get` will not store the reference with
    // the expanded lifetime. This is due to `Borrow` being overly strict and
    // can't have an impl for `&'static str` to `Borrow<&'a str>`.
    //
    //
    // See https://github.com/rust-lang/rust/issues/28853#issuecomment-158735548
    unsafe {
        let key = ::std::mem::transmute::<_, &'static str>(key);
        map.get(&UniCase::new(key)).cloned()
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

pub struct Override<'i, 'w, D> {
    valid_identifiers: HashSet<&'i str>,
    valid_words: HashSet<unicase::UniCase<&'w str>>,
    inner: D,
}

impl<'i, 'w, D: typos::Dictionary> Override<'i, 'w, D> {
    pub fn new(inner: D) -> Self {
        Self {
            valid_identifiers: Default::default(),
            valid_words: Default::default(),
            inner,
        }
    }

    pub fn valid_identifiers<I: Iterator<Item = &'i str>>(&mut self, valid_identifiers: I) {
        self.valid_identifiers = valid_identifiers.collect();
    }

    pub fn valid_words<I: Iterator<Item = &'w str>>(&mut self, valid_words: I) {
        self.valid_words = valid_words.map(UniCase::new).collect();
    }
}

impl<'i, 'w, D: typos::Dictionary> typos::Dictionary for Override<'i, 'w, D> {
    fn correct_ident<'s, 't>(&'s self, ident: typos::tokens::Identifier<'t>) -> Vec<Cow<'s, str>> {
        if self.valid_identifiers.contains(ident.token()) {
            Vec::new()
        } else {
            self.inner.correct_ident(ident)
        }
    }

    fn correct_word<'s, 't>(&'s self, word: typos::tokens::Word<'t>) -> Vec<Cow<'s, str>> {
        let w = UniCase::new(word.token());
        if self.valid_words.contains(&w) {
            Vec::new()
        } else {
            self.inner.correct_word(word)
        }
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
