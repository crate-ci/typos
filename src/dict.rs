use std::borrow::Cow;
use std::collections::HashMap;

use unicase::UniCase;

use typos::tokens::Case;
use typos::Status;

#[derive(Default)]
pub struct BuiltIn {
    locale: Option<varcon_core::Category>,
}

impl BuiltIn {
    pub const fn new(locale: crate::config::Locale) -> Self {
        Self {
            locale: locale.category(),
        }
    }

    pub fn correct_ident<'s, 'w>(
        &'s self,
        _ident: typos::tokens::Identifier<'w>,
    ) -> Option<Status<'s>> {
        None
    }

    pub fn correct_word<'s, 'w>(
        &'s self,
        word_token: typos::tokens::Word<'w>,
    ) -> Option<Status<'s>> {
        if word_token.case() == typos::tokens::Case::None {
            return None;
        }

        let word = word_token.token();
        let mut corrections = if let Some(correction) = self.correct_with_dict(word) {
            self.correct_with_vars(word)
                .unwrap_or_else(|| Status::Corrections(vec![Cow::Borrowed(correction)]))
        } else {
            self.correct_with_vars(word)?
        };
        corrections
            .corrections_mut()
            .for_each(|mut s| case_correct(&mut s, word_token.case()));
        Some(corrections)
    }

    #[cfg(feature = "dict")]
    // Not using `Status` to avoid the allocations
    fn correct_with_dict(&self, word: &str) -> Option<&'static str> {
        const WORD_RANGE: std::ops::RangeInclusive<usize> =
            typos_dict::WORD_MIN..=typos_dict::WORD_MAX;
        if WORD_RANGE.contains(&word.len()) {
            map_lookup(&typos_dict::WORD_DICTIONARY, word)
        } else {
            None
        }
    }

    #[cfg(not(feature = "dict"))]
    fn correct_with_dict(&self, _word: &str) -> Option<&'static str> {
        None
    }

    #[cfg(feature = "vars")]
    fn correct_with_vars(&self, word: &str) -> Option<Status<'static>> {
        const WORD_RANGE: std::ops::RangeInclusive<usize> =
            typos_vars::WORD_MIN..=typos_vars::WORD_MAX;
        if WORD_RANGE.contains(&word.len()) {
            map_lookup(&typos_vars::VARS_DICTIONARY, word)
                .map(|variants| self.select_variant(variants))
        } else {
            None
        }
    }

    #[cfg(not(feature = "vars"))]
    fn correct_with_vars(&self, _word: &str) -> Option<Status<'static>> {
        None
    }

    #[cfg(feature = "vars")]
    fn select_variant(
        &self,
        vars: &'static [(u8, &'static typos_vars::VariantsMap)],
    ) -> Status<'static> {
        let var = vars[0];
        let var_categories = unsafe {
            // Code-genned from a checked category-set, so known to be safe
            typos_vars::CategorySet::new(var.0)
        };
        if let Some(locale) = self.locale {
            if var_categories.contains(locale) {
                // Already valid for the current locale.
                Status::Valid
            } else {
                Status::Corrections(
                    typos_vars::corrections(locale, *var.1)
                        .iter()
                        .copied()
                        .map(Cow::Borrowed)
                        .collect(),
                )
            }
        } else {
            // All locales are valid
            if var_categories.is_empty() {
                // But the word is never valid.
                let mut unique: Vec<_> = var
                    .1
                    .iter()
                    .flat_map(|v| v.iter())
                    .copied()
                    .map(Cow::Borrowed)
                    .collect();
                unique.sort_unstable();
                unique.dedup();
                Status::Corrections(unique)
            } else {
                Status::Valid
            }
        }
    }
}

impl typos::Dictionary for BuiltIn {
    fn correct_ident<'s, 'w>(&'s self, ident: typos::tokens::Identifier<'w>) -> Option<Status<'s>> {
        BuiltIn::correct_ident(self, ident)
    }

    fn correct_word<'s, 'w>(&'s self, word: typos::tokens::Word<'w>) -> Option<Status<'s>> {
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

fn case_correct(correction: &mut Cow<'_, str>, case: Case) {
    match case {
        Case::Lower | Case::None => (),
        Case::Title => match correction {
            Cow::Borrowed(s) => {
                let mut s = String::from(*s);
                s[0..1].make_ascii_uppercase();
                *correction = s.into();
            }
            Cow::Owned(s) => {
                s[0..1].make_ascii_uppercase();
            }
        },
        Case::Upper => match correction {
            Cow::Borrowed(s) => {
                let mut s = String::from(*s);
                s.make_ascii_uppercase();
                *correction = s.into();
            }
            Cow::Owned(s) => {
                s.make_ascii_uppercase();
            }
        },
    }
}

pub struct Override<'i, 'w, D> {
    identifiers: HashMap<&'i str, Status<'i>, ahash::RandomState>,
    words: HashMap<unicase::UniCase<&'w str>, Status<'w>, ahash::RandomState>,
    inner: D,
}

impl<'i, 'w, D: typos::Dictionary> Override<'i, 'w, D> {
    pub fn new(inner: D) -> Self {
        Self {
            identifiers: Default::default(),
            words: Default::default(),
            inner,
        }
    }

    pub fn identifiers<I: Iterator<Item = (&'i str, &'i str)>>(&mut self, identifiers: I) {
        self.identifiers = Self::interpret(identifiers).collect();
    }

    pub fn words<I: Iterator<Item = (&'w str, &'w str)>>(&mut self, words: I) {
        self.words = Self::interpret(words)
            .map(|(k, v)| (UniCase::new(k), v))
            .collect();
    }

    fn interpret<'z, I: Iterator<Item = (&'z str, &'z str)>>(
        cases: I,
    ) -> impl Iterator<Item = (&'z str, Status<'z>)> {
        cases.map(|(typo, correction)| {
            let correction = if typo == correction {
                Status::Valid
            } else if correction.is_empty() {
                Status::Invalid
            } else {
                Status::Corrections(vec![Cow::Borrowed(correction)])
            };
            (typo, correction)
        })
    }
}

impl<'i, 'w, D: typos::Dictionary> typos::Dictionary for Override<'i, 'w, D> {
    fn correct_ident<'s, 't>(&'s self, ident: typos::tokens::Identifier<'t>) -> Option<Status<'s>> {
        // Skip hashing if we can
        if !self.identifiers.is_empty() {
            self.identifiers
                .get(ident.token())
                .map(|c| c.borrow())
                .or_else(|| self.inner.correct_ident(ident))
        } else {
            None
        }
    }

    fn correct_word<'s, 't>(&'s self, word: typos::tokens::Word<'t>) -> Option<Status<'s>> {
        if word.case() == typos::tokens::Case::None {
            return None;
        }

        // Skip hashing if we can
        let custom = if !self.words.is_empty() {
            let w = UniCase::new(word.token());
            // HACK: couldn't figure out the lifetime issue with replacing `cloned` with `borrow`
            self.words.get(&w).cloned()
        } else {
            None
        };
        custom.or_else(|| self.inner.correct_word(word))
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
            ("foo", Case::Upper, "FOO"),
            ("fOo", Case::None, "fOo"),
        ];
        for (correction, case, expected) in cases.iter() {
            let mut actual = Cow::Borrowed(*correction);
            case_correct(&mut actual, *case);
            assert_eq!(*expected, actual);

            let mut actual = Cow::Owned(String::from(*correction));
            case_correct(&mut actual, *case);
            assert_eq!(*expected, actual);
        }
    }
}
