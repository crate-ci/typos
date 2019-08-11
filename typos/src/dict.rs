use std::borrow::Cow;

pub trait Dictionary {
    fn correct_ident<'s, 'w>(
        &'s self,
        _ident: crate::tokens::Identifier<'w>,
    ) -> Option<Cow<'s, str>>;

    fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Option<Cow<'s, str>>;
}
