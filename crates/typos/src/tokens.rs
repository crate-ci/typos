use bstr::ByteSlice;
use winnow::BStr;

/// Define rules for tokenizaing a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenizerBuilder {
    unicode: bool,
}

impl TokenizerBuilder {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Specify that unicode Identifiers are allowed.
    #[inline]
    pub fn unicode(&mut self, yes: bool) -> &mut Self {
        self.unicode = yes;
        self
    }

    #[inline]
    pub fn build(&self) -> Tokenizer {
        let TokenizerBuilder { unicode } = self.clone();
        Tokenizer { unicode }
    }
}

impl Default for TokenizerBuilder {
    fn default() -> Self {
        Self { unicode: true }
    }
}

/// Extract Identifiers from a buffer.
#[derive(Debug, Clone)]
pub struct Tokenizer {
    unicode: bool,
}

impl Tokenizer {
    #[inline]
    pub fn new() -> Self {
        TokenizerBuilder::default().build()
    }

    pub fn parse_str<'c>(&'c self, content: &'c str) -> impl Iterator<Item = Identifier<'c>> {
        let iter = if self.unicode && !ByteSlice::is_ascii(content.as_bytes()) {
            itertools::Either::Left(unicode_parser::iter_identifiers(content))
        } else {
            itertools::Either::Right(ascii_parser::iter_identifiers(BStr::new(
                content.as_bytes(),
            )))
        };
        iter.map(move |identifier| self.transform(identifier, content.as_bytes()))
    }

    pub fn parse_bytes<'c>(&'c self, content: &'c [u8]) -> impl Iterator<Item = Identifier<'c>> {
        let iter = if self.unicode && !ByteSlice::is_ascii(content) {
            let iter = Utf8Chunks::new(content).flat_map(unicode_parser::iter_identifiers);
            itertools::Either::Left(iter)
        } else {
            itertools::Either::Right(ascii_parser::iter_identifiers(BStr::new(content)))
        };
        iter.map(move |identifier| self.transform(identifier, content))
    }

    fn transform<'i>(&self, identifier: &'i str, content: &[u8]) -> Identifier<'i> {
        debug_assert!(!identifier.is_empty());

        let case = Case::None;
        let offset = offset(content, identifier.as_bytes());
        Identifier::new_unchecked(identifier, case, offset)
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

fn offset(base: &[u8], needle: &[u8]) -> usize {
    let base = base.as_ptr() as usize;
    let needle = needle.as_ptr() as usize;
    debug_assert!(base <= needle);
    needle - base
}

struct Utf8Chunks<'s> {
    source: &'s [u8],
}

impl<'s> Utf8Chunks<'s> {
    fn new(source: &'s [u8]) -> Self {
        Self { source }
    }
}

impl<'s> Iterator for Utf8Chunks<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<&'s str> {
        if self.source.is_empty() {
            return None;
        }

        match simdutf8::compat::from_utf8(self.source) {
            Ok(valid) => {
                self.source = b"";
                Some(valid)
            }
            Err(error) => {
                let (valid, after_valid) = self.source.split_at(error.valid_up_to());

                if let Some(invalid_sequence_length) = error.error_len() {
                    self.source = &after_valid[invalid_sequence_length..];
                } else {
                    self.source = b"";
                }

                let valid = unsafe { std::str::from_utf8_unchecked(valid) };
                Some(valid)
            }
        }
    }
}

mod parser {
    use winnow::combinator::*;
    use winnow::error::ParserError;
    use winnow::prelude::*;
    use winnow::stream::AsBStr;
    use winnow::stream::AsChar;
    use winnow::stream::SliceLen;
    use winnow::stream::Stream;
    use winnow::stream::StreamIsPartial;
    use winnow::token::*;
    use winnow::trace::trace;

    pub(crate) fn next_identifier<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        preceded(ignore, identifier).parse_next(input)
    }

    fn identifier<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        // Generally a language would be `{XID_Start}{XID_Continue}*` but going with only
        // `{XID_Continue}+` because XID_Continue is a superset of XID_Start and rather catch odd
        // or unexpected cases than strip off start characters to a word since we aren't doing a
        // proper word boundary parse
        trace("identifier", take_while(1.., is_xid_continue)).parse_next(input)
    }

    fn ignore<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "ignore",
            take_many0(alt((
                // CAUTION: If adding an ignorable literal, if it doesn't start with `is_xid_continue`,
                // - Update `is_ignore_char` to make sure `sep1` doesn't eat it all up
                // - Make sure you always consume it
                terminated(uuid_literal, peek(sep1)),
                terminated(hash_literal, peek(sep1)),
                terminated(base64_literal, peek(sep1)), // base64 should be quoted or something
                terminated(ordinal_literal, peek(sep1)),
                terminated(hex_literal, peek(sep1)),
                terminated(dec_literal, peek(sep1)), // Allow digit-prefixed words
                terminated(email_literal, peek(sep1)),
                terminated(url_literal, peek(sep1)),
                terminated(css_color, peek(sep1)),
                c_escape,
                printf,
                other,
            ))),
        )
        .parse_next(input)
    }

    fn sep1<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        alt((
            one_of(|c| !is_xid_continue(c)).recognize(),
            eof.map(|_| <T as Stream>::Slice::default()),
        ))
        .parse_next(input)
    }

    fn other<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "other",
            (
                one_of(|c| !is_xid_continue(c)),
                take_while(0.., is_ignore_char),
            )
                .recognize(),
        )
        .parse_next(input)
    }

    fn ordinal_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        fn is_sep(c: impl AsChar) -> bool {
            let c = c.as_char();
            // Avoid markdown throwing off our ordinal detection
            ['_'].contains(&c)
        }

        trace(
            "ordinal_literal",
            (
                take_while(0.., is_sep),
                take_while(1.., is_dec_digit),
                alt((('s', 't'), ('n', 'd'), ('r', 'd'), ('t', 'h'))),
                take_while(0.., is_sep),
            )
                .recognize(),
        )
        .parse_next(input)
    }

    fn dec_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace("dec_literal", take_while(1.., is_dec_digit_with_sep)).parse_next(input)
    }

    fn hex_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        preceded(
            ('0', alt(('x', 'X'))),
            take_while(1.., is_hex_digit_with_sep),
        )
        .parse_next(input)
    }

    fn css_color<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "color",
            preceded(
                '#',
                alt((
                    terminated(take_while(3..=8, is_lower_hex_digit), peek(sep1)),
                    terminated(take_while(3..=8, is_upper_hex_digit), peek(sep1)),
                )),
            ),
        )
        .parse_next(input)
    }

    fn uuid_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "uuid",
            alt((
                (
                    take_while(8, is_lower_hex_digit),
                    '-',
                    take_while(4, is_lower_hex_digit),
                    '-',
                    take_while(4, is_lower_hex_digit),
                    '-',
                    take_while(4, is_lower_hex_digit),
                    '-',
                    take_while(12, is_lower_hex_digit),
                ),
                (
                    take_while(8, is_upper_hex_digit),
                    '-',
                    take_while(4, is_upper_hex_digit),
                    '-',
                    take_while(4, is_upper_hex_digit),
                    '-',
                    take_while(4, is_upper_hex_digit),
                    '-',
                    take_while(12, is_upper_hex_digit),
                ),
            ))
            .recognize(),
        )
        .parse_next(input)
    }

    fn hash_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        // Size considerations:
        //   - 40 characters holds for a SHA-1 hash from older Git versions.
        //   - 64 characters holds for a SHA-256 hash from newer Git versions.
        //   - Git allows abbreviated hashes, but we need a good abbreviation
        //     that won't be mistaken for a variable name.
        //   - Through experimentation we've found that there is almost
        //     never any actual text inside a hex string of 32 characters
        //     or more.

        const IGNORE_HEX_MIN: usize = 32;
        trace(
            "hash",
            alt((
                take_while(IGNORE_HEX_MIN.., is_lower_hex_digit),
                take_while(IGNORE_HEX_MIN.., is_upper_hex_digit),
            )),
        )
        .parse_next(input)
    }

    fn base64_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace("base64", move |input: &mut T| {
            let start = input.checkpoint();
            let captured = take_while(1.., is_base64_digit).parse_next(input)?;

            const CHUNK: usize = 4;
            let padding_offset = input.offset_from(&start);
            let mut padding_len = CHUNK - padding_offset % CHUNK;
            if padding_len == CHUNK {
                padding_len = 0;
            }

            if captured.slice_len() < 90
                && padding_len == 0
                && captured
                    .as_bstr()
                    .iter()
                    .all(|c| !['/', '+'].contains(&c.as_char()))
            {
                return Err(winnow::error::ErrMode::from_error_kind(
                    input,
                    winnow::error::ErrorKind::Slice,
                ));
            }

            take_while(padding_len..=padding_len, is_base64_padding).parse_next(input)?;

            let after_offset = input.offset_from(&start);
            input.reset(start);
            Ok(input.next_slice(after_offset))
        })
        .parse_next(input)
    }

    fn email_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "email",
            (
                take_while(1.., is_localport_char),
                '@',
                take_while(1.., is_domain_char),
            )
                .recognize(),
        )
        .parse_next(input)
    }

    fn url_literal<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "url",
            (
                opt(terminated(
                    take_while(1.., is_scheme_char),
                    // HACK: Technically you can skip `//` if you don't have a domain but that would
                    // get messy to support.
                    (':', '/', '/'),
                )),
                (
                    opt(terminated(url_userinfo, '@')),
                    take_while(1.., is_domain_char),
                    opt(preceded(':', take_while(1.., AsChar::is_dec_digit))),
                ),
                '/',
                // HACK: Too lazy to enumerate
                take_while(0.., is_path_query_fragment),
            )
                .recognize(),
        )
        .parse_next(input)
    }

    fn url_userinfo<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace(
            "userinfo",
            (
                take_while(1.., is_localport_char),
                opt(preceded(':', take_while(0.., is_localport_char))),
            )
                .recognize(),
        )
        .parse_next(input)
    }

    fn c_escape<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        // We don't know whether the string we are parsing is a literal string (no escaping) or
        // regular string that does escaping. The escaped letter might be part of a word, or it
        // might not be. Rather than guess and be wrong part of the time and correct people's words
        // incorrectly, we opt for just not evaluating it at all.
        trace(
            "escape",
            preceded(take_while(1.., is_escape), take_while(0.., is_xid_continue)),
        )
        .parse_next(input)
    }

    fn printf<T>(input: &mut T) -> PResult<<T as Stream>::Slice, ()>
    where
        T: Stream + StreamIsPartial + PartialEq,
        <T as Stream>::Slice: AsBStr + SliceLen + Default,
        <T as Stream>::Token: AsChar + Copy,
    {
        trace("printf", preceded('%', take_while(1.., is_xid_continue))).parse_next(input)
    }

    fn take_many0<I, E, F>(mut f: F) -> impl Parser<I, <I as Stream>::Slice, E>
    where
        I: Stream,
        F: Parser<I, <I as Stream>::Slice, E>,
        E: ParserError<I>,
    {
        move |i: &mut I| {
            repeat(0.., f.by_ref())
                .map(|()| ())
                .recognize()
                .parse_next(i)
        }
    }

    #[inline]
    fn is_dec_digit(i: impl AsChar + Copy) -> bool {
        i.is_dec_digit()
    }

    #[inline]
    fn is_dec_digit_with_sep(i: impl AsChar + Copy) -> bool {
        i.is_dec_digit() || is_digit_sep(i.as_char())
    }

    #[inline]
    fn is_hex_digit_with_sep(i: impl AsChar + Copy) -> bool {
        i.is_hex_digit() || is_digit_sep(i.as_char())
    }

    #[inline]
    fn is_lower_hex_digit(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        ('a'..='f').contains(&c) || c.is_ascii_digit()
    }

    #[inline]
    fn is_upper_hex_digit(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        ('A'..='F').contains(&c) || c.is_ascii_digit()
    }

    #[inline]
    fn is_base64_digit(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c.is_ascii_lowercase()
            || c.is_ascii_uppercase()
            || c.is_ascii_digit()
            || c == '+'
            || c == '/'
    }

    #[inline]
    fn is_base64_padding(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c == '='
    }

    #[inline]
    fn is_localport_char(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c.is_ascii_lowercase()
            || c.is_ascii_uppercase()
            || c.is_ascii_digit()
            || "!#$%&'*+-/=?^_`{|}~().".find(c).is_some()
    }

    #[inline]
    fn is_domain_char(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c.is_ascii_lowercase()
            || c.is_ascii_uppercase()
            || c.is_ascii_digit()
            || "-().".find(c).is_some()
    }

    #[inline]
    fn is_path_query_fragment(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        is_pchar(c) || "/?#".find(c).is_some()
    }

    #[inline]
    fn is_pchar(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        is_uri_unreserved(c) || is_uri_sub_delims(c) || "%:@".find(c).is_some()
    }

    #[inline]
    fn is_uri_unreserved(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c.is_ascii_lowercase()
            || c.is_ascii_uppercase()
            || c.is_ascii_digit()
            || "-._~".find(c).is_some()
    }

    #[inline]
    fn is_uri_sub_delims(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        "!$&'()*+,;=".find(c).is_some()
    }

    #[inline]
    fn is_scheme_char(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c.is_ascii_lowercase() || c.is_ascii_digit() || "+.-".find(c).is_some()
    }

    #[inline]
    fn is_ignore_char(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        !unicode_xid::UnicodeXID::is_xid_continue(c) &&
            // See c_escape
            c != '\\' &&
            // See printf
            c != '%' &&
            // See css_color
            c != '#'
    }

    #[inline]
    fn is_xid_continue(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        unicode_xid::UnicodeXID::is_xid_continue(c)
    }

    #[inline]
    fn is_escape(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        c == '\\'
    }

    #[inline]
    fn is_digit_sep(chr: char) -> bool {
        // `_`: number literal separator in Rust and other languages
        // `'`: number literal separator in C++
        chr == '_' || chr == '\''
    }
}

mod unicode_parser {
    use super::parser::next_identifier;

    pub(crate) fn iter_identifiers(mut input: &str) -> impl Iterator<Item = &str> {
        std::iter::from_fn(move || match next_identifier(&mut input) {
            Ok(o) => {
                debug_assert_ne!(o, "");
                Some(o)
            }
            _ => None,
        })
    }
}

mod ascii_parser {
    use super::parser::next_identifier;

    use winnow::BStr;

    pub(crate) fn iter_identifiers(mut input: &BStr) -> impl Iterator<Item = &str> {
        std::iter::from_fn(move || match next_identifier(&mut input) {
            Ok(o) => {
                debug_assert_ne!(o, b"");
                // This is safe because we've checked that the strings are a subset of ASCII
                // characters.
                let o = unsafe { std::str::from_utf8_unchecked(o) };
                Some(o)
            }
            _ => None,
        })
    }
}

/// A term composed of Words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'t> {
    token: &'t str,
    case: Case,
    offset: usize,
}

impl<'t> Identifier<'t> {
    #[inline]
    pub fn new_unchecked(token: &'t str, case: Case, offset: usize) -> Self {
        Self {
            token,
            case,
            offset,
        }
    }

    #[inline]
    pub fn token(&self) -> &'t str {
        self.token
    }

    #[inline]
    pub fn case(&self) -> Case {
        self.case
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn span(&self) -> std::ops::Range<usize> {
        let start = self.offset;
        let end = start + self.token.len();
        start..end
    }

    /// Split into individual Words.
    #[inline]
    pub fn split(&self) -> impl Iterator<Item = Word<'t>> {
        match self.case {
            Case::None => itertools::Either::Left(SplitIdent::new(self.token, self.offset)),
            _ => itertools::Either::Right(
                Some(Word::new_unchecked(self.token, self.case, self.offset)).into_iter(),
            ),
        }
    }
}

/// An indivisible term.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word<'t> {
    token: &'t str,
    case: Case,
    offset: usize,
}

impl<'t> Word<'t> {
    pub fn new(token: &'t str, offset: usize) -> Result<Self, std::io::Error> {
        let mut itr = SplitIdent::new(token, 0);
        let mut item = itr.next().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{:?} is nothing", token),
            )
        })?;
        if item.offset != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{:?} has padding", token),
            ));
        }
        item.offset += offset;
        if itr.next().is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{:?} is multiple words", token),
            ));
        }
        Ok(item)
    }

    #[inline]
    pub fn new_unchecked(token: &'t str, case: Case, offset: usize) -> Self {
        Self {
            token,
            case,
            offset,
        }
    }

    #[inline]
    pub fn token(&self) -> &'t str {
        self.token
    }

    #[inline]
    pub fn case(&self) -> Case {
        self.case
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn span(&self) -> std::ops::Range<usize> {
        let start = self.offset;
        let end = start + self.token.len();
        start..end
    }
}

struct SplitIdent<'s> {
    ident: &'s str,
    offset: usize,

    char_indices: std::iter::Peekable<std::str::CharIndices<'s>>,
    start: usize,
    start_mode: WordMode,
    last_mode: WordMode,
}

impl<'s> SplitIdent<'s> {
    fn new(ident: &'s str, offset: usize) -> Self {
        Self {
            ident,
            offset,
            char_indices: ident.char_indices().peekable(),
            start: 0,
            start_mode: WordMode::Boundary,
            last_mode: WordMode::Boundary,
        }
    }
}

impl<'s> Iterator for SplitIdent<'s> {
    type Item = Word<'s>;

    fn next(&mut self) -> Option<Word<'s>> {
        #[allow(clippy::while_let_on_iterator)]
        while let Some((i, c)) = self.char_indices.next() {
            let cur_mode = WordMode::classify(c);
            if cur_mode == WordMode::Boundary {
                debug_assert!(self.start_mode == WordMode::Boundary);
                continue;
            }
            if self.start_mode == WordMode::Boundary {
                self.start_mode = cur_mode;
                self.start = i;
            }

            if let Some(&(next_i, next)) = self.char_indices.peek() {
                // The mode including the current character, assuming the current character does
                // not result in a word boundary.
                let next_mode = WordMode::classify(next);

                match (self.last_mode, cur_mode, next_mode) {
                    // cur_mode is last of current word
                    (_, _, WordMode::Boundary)
                    | (_, WordMode::Lowercase, WordMode::Number)
                    | (_, WordMode::Uppercase, WordMode::Number)
                    | (_, WordMode::Number, WordMode::Lowercase)
                    | (_, WordMode::Number, WordMode::Uppercase)
                    | (_, WordMode::Lowercase, WordMode::Uppercase) => {
                        let case = self.start_mode.case(cur_mode);
                        let result = Word::new_unchecked(
                            &self.ident[self.start..next_i],
                            case,
                            self.start + self.offset,
                        );
                        self.start = next_i;
                        self.start_mode = WordMode::Boundary;
                        self.last_mode = WordMode::Boundary;
                        return Some(result);
                    }
                    // cur_mode is start of next word
                    (WordMode::Uppercase, WordMode::Uppercase, WordMode::Lowercase) => {
                        let result = Word::new_unchecked(
                            &self.ident[self.start..i],
                            Case::Upper,
                            self.start + self.offset,
                        );
                        self.start = i;
                        self.start_mode = cur_mode;
                        self.last_mode = WordMode::Boundary;
                        return Some(result);
                    }
                    // No word boundary
                    (_, _, _) => {
                        self.last_mode = cur_mode;
                    }
                }
            } else {
                // Collect trailing characters as a word
                let case = self.start_mode.case(cur_mode);
                let result =
                    Word::new_unchecked(&self.ident[self.start..], case, self.start + self.offset);
                return Some(result);
            }
        }

        None
    }
}

/// Format of the term.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Title,
    Lower,
    Upper,
    None,
}

/// Tracks the current 'mode' of the transformation algorithm as it scans the input string.
///
/// The mode is a tri-state which tracks the case of the last cased character of the current
/// word. If there is no cased character (either lowercase or uppercase) since the previous
/// word boundary, than the mode is `Boundary`. If the last cased character is lowercase, then
/// the mode is `Lowercase`. Otherrwise, the mode is `Uppercase`.
#[derive(Clone, Copy, PartialEq, Debug)]
enum WordMode {
    /// There have been no lowercase or uppercase characters in the current word.
    Boundary,
    /// The previous cased character in the current word is lowercase.
    Lowercase,
    /// The previous cased character in the current word is uppercase.
    Uppercase,
    Number,
}

impl WordMode {
    fn classify(c: char) -> Self {
        if c.is_lowercase() {
            WordMode::Lowercase
        } else if c.is_uppercase() {
            WordMode::Uppercase
        } else if c.is_ascii_digit() {
            WordMode::Number
        } else {
            // This assumes all characters are either lower or upper case.
            WordMode::Boundary
        }
    }

    fn case(self, last: WordMode) -> Case {
        match (self, last) {
            (WordMode::Uppercase, WordMode::Uppercase) => Case::Upper,
            (WordMode::Uppercase, WordMode::Lowercase) => Case::Title,
            (WordMode::Lowercase, WordMode::Lowercase) => Case::Lower,
            (WordMode::Number, WordMode::Number) => Case::None,
            (WordMode::Number, _)
            | (_, WordMode::Number)
            | (WordMode::Boundary, _)
            | (_, WordMode::Boundary)
            | (WordMode::Lowercase, WordMode::Uppercase) => {
                unreachable!("Invalid case combination: ({:?}, {:?})", self, last)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_empty_is_empty() {
        let parser = Tokenizer::new();

        let input = "";
        let expected: Vec<Identifier> = vec![];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_word_is_word() {
        let parser = Tokenizer::new();

        let input = "word";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("word", Case::None, 0)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_space_separated_words() {
        let parser = Tokenizer::new();

        let input = "A B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", Case::None, 0),
            Identifier::new_unchecked("B", Case::None, 2),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_dot_separated_words() {
        let parser = Tokenizer::new();

        let input = "A.B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", Case::None, 0),
            Identifier::new_unchecked("B", Case::None, 2),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_namespace_separated_words() {
        let parser = Tokenizer::new();

        let input = "A::B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", Case::None, 0),
            Identifier::new_unchecked("B", Case::None, 3),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let parser = Tokenizer::new();

        let input = "A_B";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("A_B", Case::None, 0)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_ordinal() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello 1st 2nd 3rd 4th __5th__ World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 30),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_hex() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 17),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_uuid() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello 123e4567-e89b-12d3-a456-426652340000 World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 43),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_uuid_uppercase() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello 123E4567-E89B-12D3-A456-426652340000 World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 43),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_hash() {
        let parser = TokenizerBuilder::new().build();

        for (hashlike, is_ignored) in [
            // A SHA-1 output, in lower case.
            ("485865fd0412e40d041e861506bb3ac11a3a91e3", true),
            // A SHA-1 output, in mixed case: Not a ignored.
            ("485865fd0412e40d041E861506BB3AC11A3A91E3", false),
            // A SHA-256 output, in upper case.
            ("E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855", true),
            // A SHA-512 output, in lower case.
            ("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e", true),
            // An MD5 (deprecated) output, in upper case.
            ("D41D8CD98F00B204E9800998ECF8427E", true),
            // A 31-character hexadecimal string: too short to be a hash.
            ("D41D8CD98F00B204E9800998ECF8427", false),
        ] {
            let input = format!("Hello {} World", hashlike);
            let mut expected: Vec<Identifier> = vec![
                Identifier::new_unchecked("Hello", Case::None, 0),
                Identifier::new_unchecked("World", Case::None, 7+hashlike.len()),
            ];
            if ! is_ignored {
                expected.insert(1, Identifier::new_unchecked(hashlike, Case::None, 6));
            }
            let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
            assert_eq!(expected, actual);
            let actual: Vec<_> = parser.parse_str(&input).collect();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn tokenize_hash_in_mixed_path() {
        let parser = TokenizerBuilder::new().build();

        let input = "     ///                 at /rustc/c7087fe00d2ba919df1d813c040a5d47e43b0fe7\\/src\\libstd\\rt.rs:51";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("at", Case::None, 25),
            // `rustc...` looks like the start of a URL
            Identifier::new_unchecked("rs", Case::None, 91),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_base64_case_1() {
        let parser = TokenizerBuilder::new().build();

        let input = "Good Iy9+btvut+d92V+v84444ziIqJKHK879KJH59//X1Iy9+btvut+d92V+v84444ziIqJKHK879KJH59//X122Iy9+btvut+d92V+v84444ziIqJKHK879KJH59//X12== Bye";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Good", Case::None, 0),
            Identifier::new_unchecked("Bye", Case::None, 134),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_base64_case_2() {
        let parser = TokenizerBuilder::new().build();

        let input = r#""ed25519:1": "Wm+VzmOUOz08Ds+0NTWb1d4CZrVsJSikkeRxh6aCcUwu6pNC78FunoD7KNWzqFn241eYHYMGCA5McEiVPdhzBA==""#;
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("ed25519", Case::None, 1)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_base64_case_3() {
        let parser = TokenizerBuilder::new().build();

        let input = r#"       "integrity": "sha512-hCmlUAIlUiav8Xdqw3Io4LcpA1DOt7h3LSTAC4G6JGHFFaWzI6qvFt9oilvl8BmkbBRX1IhM90ZAmpk68zccQA==","#;
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("integrity", Case::None, 8),
            Identifier::new_unchecked("sha512", Case::None, 21),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_email() {
        let parser = TokenizerBuilder::new().build();

        let input = "Good example@example.com Bye";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Good", Case::None, 0),
            Identifier::new_unchecked("Bye", Case::None, 25),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_min_url() {
        let parser = TokenizerBuilder::new().build();

        let input = "Good example.com/hello Bye";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Good", Case::None, 0),
            Identifier::new_unchecked("Bye", Case::None, 23),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_max_url() {
        let parser = TokenizerBuilder::new().build();

        let input =
            "Good http://user:password@example.com:3142/hello?query=value&extra=two#fragment,split Bye";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Good", Case::None, 0),
            Identifier::new_unchecked("Bye", Case::None, 86),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_leading_digits() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello 0Hello 124 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("0Hello", Case::None, 6),
            Identifier::new_unchecked("World", Case::None, 28),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_c_escape() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello \\Hello \\ \\\\ World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 18),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_double_escape() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello \\n\\n World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 11),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_escape() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello \\nanana\\nanana World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 21),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_printf() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello %Hello World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("World", Case::None, 13),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_color() {
        let parser = TokenizerBuilder::new().build();

        let input = "#[derive(Clone)] #aaa # #111 #AABBCC #hello #AABBCCDD #1175BA World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("derive", Case::None, 2),
            Identifier::new_unchecked("Clone", Case::None, 9),
            Identifier::new_unchecked("hello", Case::None, 38),
            Identifier::new_unchecked("World", Case::None, 62),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_template() {
        let parser = TokenizerBuilder::new().build();

        let input = "Hello {{% foo %}} world!";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("foo", Case::None, 10),
            Identifier::new_unchecked("world", Case::None, 18),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn split_ident() {
        let cases = [
            (
                "lowercase",
                &[("lowercase", Case::Lower, 0usize)] as &[(&str, Case, usize)],
            ),
            ("Class", &[("Class", Case::Title, 0)]),
            (
                "MyClass",
                &[("My", Case::Title, 0), ("Class", Case::Title, 2)],
            ),
            ("MyC", &[("My", Case::Title, 0), ("C", Case::Upper, 2)]),
            ("HTML", &[("HTML", Case::Upper, 0)]),
            (
                "PDFLoader",
                &[("PDF", Case::Upper, 0), ("Loader", Case::Title, 3)],
            ),
            (
                "AString",
                &[("A", Case::Upper, 0), ("String", Case::Title, 1)],
            ),
            (
                "SimpleXMLTokenizer",
                &[
                    ("Simple", Case::Title, 0),
                    ("XML", Case::Upper, 6),
                    ("Tokenizer", Case::Title, 9),
                ],
            ),
            (
                "vimRPCPlugin",
                &[
                    ("vim", Case::Lower, 0),
                    ("RPC", Case::Upper, 3),
                    ("Plugin", Case::Title, 6),
                ],
            ),
            (
                "GL11Version",
                &[
                    ("GL", Case::Upper, 0),
                    ("11", Case::None, 2),
                    ("Version", Case::Title, 4),
                ],
            ),
            (
                "99Bottles",
                &[("99", Case::None, 0), ("Bottles", Case::Title, 2)],
            ),
            ("May5", &[("May", Case::Title, 0), ("5", Case::None, 3)]),
            (
                "BFG9000",
                &[("BFG", Case::Upper, 0), ("9000", Case::None, 3)],
            ),
        ];
        for (input, expected) in cases.iter() {
            let ident = Identifier::new_unchecked(input, Case::None, 0);
            let result: Vec<_> = ident.split().map(|w| (w.token, w.case, w.offset)).collect();
            assert_eq!(&result, expected);
        }
    }
}
