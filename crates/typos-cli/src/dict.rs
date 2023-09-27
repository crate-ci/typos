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

    pub fn correct_ident<'s>(
        &'s self,
        ident_token: typos::tokens::Identifier<'_>,
    ) -> Option<Status<'s>> {
        let ident = ident_token.token();
        self.correct_ident_with_dict(ident)
    }

    pub fn correct_word<'s>(&'s self, word_token: typos::tokens::Word<'_>) -> Option<Status<'s>> {
        if word_token.case() == typos::tokens::Case::None {
            return None;
        }

        let word = word_token.token();
        let word_case = unicase::UniCase::new(word);
        let mut corrections = if let Some(corrections) = self.correct_word_with_dict(word_case) {
            if corrections.is_empty() {
                Status::Invalid
            } else {
                self.chain_with_vars(corrections)
            }
        } else {
            self.correct_with_vars(word_case)?
        };
        for s in corrections.corrections_mut() {
            case_correct(s, word_token.case())
        }
        Some(corrections)
    }
}

#[cfg(feature = "dict")]
impl BuiltIn {
    fn correct_ident_with_dict<'s>(&self, ident: &str) -> Option<Status<'s>> {
        match ident {
            "O_WRONLY" => Some(Status::Valid),
            _ => None,
        }
    }

    // Not using `Status` to avoid the allocations
    fn correct_word_with_dict(
        &self,
        word: unicase::UniCase<&str>,
    ) -> Option<&'static [&'static str]> {
        typos_dict::WORD_TRIE.find(&word).copied()
    }
}

#[cfg(not(feature = "dict"))]
impl BuiltIn {
    fn correct_ident_with_dict<'s>(&self, _ident: &str) -> Option<Status<'s>> {
        None
    }

    fn correct_word_with_dict(
        &self,
        _word: unicase::UniCase<&str>,
    ) -> Option<&'static [&'static str]> {
        None
    }
}

#[cfg(feature = "vars")]
impl BuiltIn {
    fn chain_with_vars(&self, corrections: &'static [&'static str]) -> Status<'static> {
        if self.is_vars_enabled() {
            let mut chained: Vec<_> = corrections
                .iter()
                .flat_map(|c| match self.correct_with_vars(unicase::UniCase::new(c)) {
                    Some(Status::Valid) | None => vec![Cow::Borrowed(*c)],
                    Some(Status::Corrections(vars)) => vars,
                    Some(Status::Invalid) => {
                        unreachable!("correct_with_vars should always have valid suggestions")
                    }
                })
                .collect();
            if chained.len() != 1 {
                chained.sort_unstable();
                chained.dedup();
            }
            debug_assert!(!chained.is_empty());
            Status::Corrections(chained)
        } else {
            Status::Corrections(corrections.iter().map(|c| Cow::Borrowed(*c)).collect())
        }
    }

    fn correct_with_vars(&self, word: unicase::UniCase<&str>) -> Option<Status<'static>> {
        if self.is_vars_enabled() {
            typos_vars::VARS_TRIE
                .find(&word)
                .map(|variants| self.select_variant(variants))
        } else {
            None
        }
    }

    fn is_vars_enabled(&self) -> bool {
        #![allow(clippy::assertions_on_constants)]
        debug_assert!(typos_vars::NO_INVALID);
        self.locale.is_some()
    }

    fn select_variant(
        &self,
        vars: &'static [(u8, &'static typos_vars::VariantsMap)],
    ) -> Status<'static> {
        let var = vars[0];
        let var_categories = unsafe {
            // Code-genned from a checked category-set, so known to be safe
            typos_vars::CategorySet::from_bits_unchecked(var.0)
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

#[cfg(not(feature = "vars"))]
impl BuiltIn {
    fn chain_with_vars(&self, corrections: &'static [&'static str]) -> Status<'static> {
        Status::Corrections(corrections.iter().map(|c| Cow::Borrowed(*c)).collect())
    }

    fn correct_with_vars(&self, _word: unicase::UniCase<&str>) -> Option<Status<'static>> {
        None
    }
}

impl typos::Dictionary for BuiltIn {
    fn correct_ident<'s>(&'s self, ident: typos::tokens::Identifier<'_>) -> Option<Status<'s>> {
        BuiltIn::correct_ident(self, ident)
    }

    fn correct_word<'s>(&'s self, word: typos::tokens::Word<'_>) -> Option<Status<'s>> {
        BuiltIn::correct_word(self, word)
    }
}

#[allow(clippy::ptr_arg)]
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
    ignored_identifiers: Vec<regex::Regex>,
    identifiers: HashMap<&'i str, Status<'i>, ahash::RandomState>,
    words: HashMap<unicase::UniCase<&'w str>, Status<'w>, ahash::RandomState>,
    inner: D,
}

impl<'i, 'w, D: typos::Dictionary> Override<'i, 'w, D> {
    pub fn new(inner: D) -> Self {
        Self {
            ignored_identifiers: Default::default(),
            identifiers: Default::default(),
            words: Default::default(),
            inner,
        }
    }

    pub fn ignored_identifiers<'r>(&mut self, ignored: impl Iterator<Item = &'r regex::Regex>) {
        self.ignored_identifiers.extend(ignored.cloned());
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
    fn correct_ident<'s>(&'s self, ident: typos::tokens::Identifier<'_>) -> Option<Status<'s>> {
        for ignored in &self.ignored_identifiers {
            if ignored.is_match(ident.token()) {
                return Some(Status::Valid);
            }
        }

        // Skip hashing if we can
        if !self.identifiers.is_empty() {
            if let Some(status) = self.identifiers.get(ident.token()).map(|c| c.borrow()) {
                return Some(status);
            }
        }

        self.inner.correct_ident(ident)
    }

    fn correct_word<'s>(&'s self, word: typos::tokens::Word<'_>) -> Option<Status<'s>> {
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

    #[cfg(feature = "dict")]
    #[test]
    fn test_dict_correct() {
        let dict = BuiltIn::new(crate::config::Locale::default());
        let correction = dict.correct_word(typos::tokens::Word::new_unchecked(
            "finallizes",
            typos::tokens::Case::Lower,
            0,
        ));
        assert_eq!(
            correction,
            Some(Status::Corrections(vec!["finalizes".into()]))
        );
    }

    #[cfg(feature = "vars")]
    #[test]
    fn test_varcon_no_locale() {
        let dict = BuiltIn::new(crate::config::Locale::En);
        let correction = dict.correct_word(typos::tokens::Word::new_unchecked(
            "finalizes",
            typos::tokens::Case::Lower,
            0,
        ));
        assert_eq!(correction, None);
    }

    #[cfg(feature = "vars")]
    #[test]
    fn test_varcon_same_locale() {
        let dict = BuiltIn::new(crate::config::Locale::EnUs);
        let correction = dict.correct_word(typos::tokens::Word::new_unchecked(
            "finalizes",
            typos::tokens::Case::Lower,
            0,
        ));
        assert_eq!(correction, Some(Status::Valid));
    }

    #[cfg(feature = "vars")]
    #[test]
    fn test_varcon_different_locale() {
        let dict = BuiltIn::new(crate::config::Locale::EnGb);
        let correction = dict.correct_word(typos::tokens::Word::new_unchecked(
            "finalizes",
            typos::tokens::Case::Lower,
            0,
        ));
        assert_eq!(
            correction,
            Some(Status::Corrections(vec!["finalises".into()]))
        );
    }

    #[cfg(all(feature = "dict", feature = "vars"))]
    #[test]
    fn test_dict_to_varcon() {
        let dict = BuiltIn::new(crate::config::Locale::EnGb);
        let correction = dict.correct_word(typos::tokens::Word::new_unchecked(
            "finallizes",
            typos::tokens::Case::Lower,
            0,
        ));
        assert_eq!(
            correction,
            Some(Status::Corrections(vec!["finalises".into()]))
        );
    }

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
