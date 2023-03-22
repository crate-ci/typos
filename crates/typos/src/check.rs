use crate::tokens;
use crate::Dictionary;
use std::borrow::Cow;

pub fn check_str<'b, 's: 'b>(
    buffer: &'b str,
    tokenizer: &'s tokens::Tokenizer,
    dictionary: &'s dyn Dictionary,
) -> impl Iterator<Item = Typo<'b>> {
    tokenizer
        .parse_str(buffer)
        .flat_map(move |ident| process_ident(ident, dictionary))
}

pub fn check_bytes<'b, 's: 'b>(
    buffer: &'b [u8],
    tokenizer: &'s tokens::Tokenizer,
    dictionary: &'s dyn Dictionary,
) -> impl Iterator<Item = Typo<'b>> {
    tokenizer
        .parse_bytes(buffer)
        .flat_map(move |ident| process_ident(ident, dictionary))
}

fn process_ident<'i, 's: 'i>(
    ident: tokens::Identifier<'i>,
    dictionary: &'s dyn Dictionary,
) -> impl Iterator<Item = Typo<'i>> {
    match dictionary.correct_ident(ident) {
        Some(crate::Status::Valid) => itertools::Either::Left(None.into_iter()),
        Some(corrections) => {
            let typo = Typo {
                byte_offset: ident.offset(),
                typo: ident.token().into(),
                corrections,
            };
            itertools::Either::Left(Some(typo).into_iter())
        }
        None => itertools::Either::Right(
            ident
                .split()
                .filter_map(move |word| process_word(word, dictionary)),
        ),
    }
}

fn process_word<'w, 's: 'w>(
    word: tokens::Word<'w>,
    dictionary: &'s dyn Dictionary,
) -> Option<Typo<'w>> {
    match dictionary.correct_word(word) {
        Some(crate::Status::Valid) => None,
        Some(corrections) => {
            let typo = Typo {
                byte_offset: word.offset(),
                typo: word.token().into(),
                corrections,
            };
            Some(typo)
        }
        None => None,
    }
}

/// An invalid term found in the buffer.
#[derive(Clone, Debug)]
pub struct Typo<'m> {
    pub byte_offset: usize,
    pub typo: Cow<'m, str>,
    pub corrections: crate::Status<'m>,
}

impl<'m> Typo<'m> {
    pub fn into_owned(self) -> Typo<'static> {
        Typo {
            byte_offset: self.byte_offset,
            typo: Cow::Owned(self.typo.into_owned()),
            corrections: self.corrections.into_owned(),
        }
    }

    pub fn borrow(&self) -> Typo<'_> {
        Typo {
            byte_offset: self.byte_offset,
            typo: Cow::Borrowed(self.typo.as_ref()),
            corrections: self.corrections.borrow(),
        }
    }

    pub fn span(&self) -> std::ops::Range<usize> {
        let start = self.byte_offset;
        let end = start + self.typo.len();
        start..end
    }
}

impl<'m> Default for Typo<'m> {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            typo: "".into(),
            corrections: crate::Status::Invalid,
        }
    }
}
