use std::borrow::Cow;

pub trait Dictionary: Send + Sync {
    fn correct_ident<'s, 'w>(&'s self, _ident: crate::tokens::Identifier<'w>) -> Vec<Cow<'s, str>>;

    fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Vec<Cow<'s, str>>;
}
