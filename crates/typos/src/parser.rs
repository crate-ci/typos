use crate::tokens;
use crate::Dictionary;

#[derive(Clone)]
pub struct ParserBuilder<'p, 'd> {
    tokenizer: Option<&'p tokens::Tokenizer>,
    dictionary: &'d dyn Dictionary,
}

impl<'p> ParserBuilder<'p, 'static> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'p, 'd> ParserBuilder<'p, 'd> {
    pub fn tokenizer(mut self, tokenizer: &'p tokens::Tokenizer) -> Self {
        self.tokenizer = Some(tokenizer);
        self
    }

    pub fn dictionary<'d1>(self, dictionary: &'d1 dyn Dictionary) -> ParserBuilder<'p, 'd1> {
        ParserBuilder {
            tokenizer: self.tokenizer,
            dictionary: dictionary,
        }
    }

    pub fn typos(&self) -> TyposParser<'p, 'd> {
        TyposParser {
            tokenizer: self.tokenizer.unwrap_or_else(|| &DEFAULT_TOKENIZER),
            dictionary: self.dictionary,
        }
    }

    pub fn identifiers(&self) -> IdentifiersParser<'p> {
        IdentifiersParser {
            tokenizer: self.tokenizer.unwrap_or_else(|| &DEFAULT_TOKENIZER),
        }
    }

    pub fn words(&self) -> WordsParser<'p> {
        WordsParser {
            tokenizer: self.tokenizer.unwrap_or_else(|| &DEFAULT_TOKENIZER),
        }
    }
}

impl<'p> Default for ParserBuilder<'p, 'static> {
    fn default() -> Self {
        Self {
            tokenizer: None,
            dictionary: &crate::NullDictionary,
        }
    }
}

static DEFAULT_TOKENIZER: once_cell::sync::Lazy<tokens::Tokenizer> =
    once_cell::sync::Lazy::new(|| tokens::Tokenizer::new());

#[derive(Clone)]
pub struct TyposParser<'p, 'd> {
    tokenizer: &'p tokens::Tokenizer,
    dictionary: &'d dyn Dictionary,
}

impl<'p, 'd> TyposParser<'p, 'd> {
    pub fn parse_str<'b, 's: 'b>(&'s self, buffer: &'b str) -> impl Iterator<Item = Typo<'b>> {
        self.tokenizer
            .parse_str(buffer)
            .flat_map(move |ident| self.process_ident(ident))
    }

    pub fn parse_bytes<'b, 's: 'b>(&'s self, buffer: &'b [u8]) -> impl Iterator<Item = Typo<'b>> {
        self.tokenizer
            .parse_bytes(buffer)
            .flat_map(move |ident| self.process_ident(ident))
    }

    fn process_ident<'i, 's: 'i>(
        &'s self,
        ident: tokens::Identifier<'i>,
    ) -> impl Iterator<Item = Typo<'i>> {
        match self.dictionary.correct_ident(ident) {
            Some(crate::Status::Valid) => itertools::Either::Left(None.into_iter()),
            Some(corrections) => {
                let typo = Typo {
                    byte_offset: ident.offset(),
                    typo: ident.token(),
                    corrections,
                };
                itertools::Either::Left(Some(typo).into_iter())
            }
            None => itertools::Either::Right(
                ident
                    .split()
                    .filter_map(move |word| self.process_word(word)),
            ),
        }
    }

    fn process_word<'w, 's: 'w>(&'s self, word: tokens::Word<'w>) -> Option<Typo<'w>> {
        match self.dictionary.correct_word(word) {
            Some(crate::Status::Valid) => None,
            Some(corrections) => {
                let typo = Typo {
                    byte_offset: word.offset(),
                    typo: word.token(),
                    corrections,
                };
                Some(typo)
            }
            None => None,
        }
    }
}

#[derive(Clone, Debug, derive_setters::Setters)]
#[non_exhaustive]
pub struct Typo<'m> {
    pub byte_offset: usize,
    pub typo: &'m str,
    pub corrections: crate::Status<'m>,
}

impl<'m> Default for Typo<'m> {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            typo: "",
            corrections: crate::Status::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifiersParser<'p> {
    tokenizer: &'p tokens::Tokenizer,
}

impl<'p> IdentifiersParser<'p> {
    pub fn parse_str(&self, buffer: &'p str) -> impl Iterator<Item = tokens::Identifier<'p>> {
        self.tokenizer.parse_str(buffer)
    }

    pub fn parse_bytes(&self, buffer: &'p [u8]) -> impl Iterator<Item = tokens::Identifier<'p>> {
        self.tokenizer.parse_bytes(buffer)
    }
}

#[derive(Debug, Clone)]
pub struct WordsParser<'p> {
    tokenizer: &'p tokens::Tokenizer,
}

impl<'p> WordsParser<'p> {
    pub fn parse_str(&self, buffer: &'p str) -> impl Iterator<Item = tokens::Word<'p>> {
        self.tokenizer.parse_str(buffer).flat_map(|i| i.split())
    }

    pub fn parse_bytes(&self, buffer: &'p [u8]) -> impl Iterator<Item = tokens::Word<'p>> {
        self.tokenizer.parse_bytes(buffer).flat_map(|i| i.split())
    }
}
