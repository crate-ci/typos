use bstr::ByteSlice;

/// Define rules for tokenizaing a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenizerBuilder {
    unicode: bool,
}

impl TokenizerBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// Specify that unicode Identifiers are allowed.
    pub fn unicode(&mut self, yes: bool) -> &mut Self {
        self.unicode = yes;
        self
    }

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
    pub fn new() -> Self {
        TokenizerBuilder::default().build()
    }

    pub fn parse_str<'c>(&'c self, content: &'c str) -> impl Iterator<Item = Identifier<'c>> {
        let iter = if self.unicode && !ByteSlice::is_ascii(content.as_bytes()) {
            itertools::Either::Left(unicode_parser::iter_identifiers(content))
        } else {
            itertools::Either::Right(ascii_parser::iter_identifiers(content.as_bytes()))
        };
        iter.filter_map(move |identifier| {
            let offset = offset(content.as_bytes(), identifier.as_bytes());
            self.transform(identifier, offset)
        })
    }

    pub fn parse_bytes<'c>(&'c self, content: &'c [u8]) -> impl Iterator<Item = Identifier<'c>> {
        let iter = if self.unicode && !ByteSlice::is_ascii(content) {
            let iter =
                Utf8Chunks::new(content).flat_map(move |c| unicode_parser::iter_identifiers(c));
            itertools::Either::Left(iter)
        } else {
            itertools::Either::Right(ascii_parser::iter_identifiers(content))
        };
        iter.filter_map(move |identifier| {
            let offset = offset(content, identifier.as_bytes());
            self.transform(identifier, offset)
        })
    }

    fn transform<'i>(&self, identifier: &'i str, offset: usize) -> Option<Identifier<'i>> {
        debug_assert!(!identifier.is_empty());

        let case = Case::None;
        Some(Identifier::new_unchecked(identifier, case, offset))
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
    use nom::branch::*;
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::combinator::*;
    use nom::sequence::*;
    use nom::{AsChar, IResult};

    pub(crate) fn next_identifier<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition
            + nom::InputTake
            + nom::InputIter
            + nom::InputLength
            + nom::Slice<std::ops::RangeFrom<usize>>
            + nom::Slice<std::ops::RangeTo<usize>>
            + nom::Offset
            + Clone
            + PartialEq
            + std::fmt::Debug,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
        <T as nom::InputIter>::Item: AsChar + Copy,
    {
        preceded(ignore, identifier)(input)
    }

    fn identifier<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
    {
        // Generally a language would be `{XID_Start}{XID_Continue}*` but going with only
        // `{XID_Continue}+` because XID_Continue is a superset of XID_Start and rather catch odd
        // or unexpected cases than strip off start characters to a word since we aren't doing a
        // proper word boundary parse
        take_while1(is_xid_continue)(input)
    }

    fn ignore<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition
            + nom::InputTake
            + nom::InputIter
            + nom::InputLength
            + nom::Slice<std::ops::RangeFrom<usize>>
            + nom::Slice<std::ops::RangeTo<usize>>
            + nom::Offset
            + Clone
            + PartialEq
            + std::fmt::Debug,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
        <T as nom::InputIter>::Item: AsChar + Copy,
    {
        take_many0(alt((
            sep1,
            terminated(uuid_literal, sep1),
            terminated(hex_literal, sep1),
            terminated(dec_literal, sep1),
        )))(input)
    }

    fn sep1<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
    {
        take_till1(is_xid_continue)(input)
    }

    fn dec_literal<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
    {
        take_while1(is_dec_digit_with_sep)(input)
    }

    fn hex_literal<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition
            + nom::InputTake
            + nom::InputIter
            + nom::InputLength
            + nom::Slice<std::ops::RangeFrom<usize>>
            + Clone,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
        <T as nom::InputIter>::Item: AsChar + Copy,
    {
        preceded(
            pair(char('0'), alt((char('x'), char('X')))),
            take_while1(is_hex_digit_with_sep),
        )(input)
    }

    fn uuid_literal<T>(input: T) -> IResult<T, T>
    where
        T: nom::InputTakeAtPosition
            + nom::InputTake
            + nom::InputIter
            + nom::InputLength
            + nom::Offset
            + nom::Slice<std::ops::RangeTo<usize>>
            + nom::Slice<std::ops::RangeFrom<usize>>
            + Clone,
        <T as nom::InputTakeAtPosition>::Item: AsChar + Copy,
        <T as nom::InputIter>::Item: AsChar + Copy,
    {
        recognize(tuple((
            take_while_m_n(8, 8, is_lower_hex_digit),
            char('-'),
            take_while_m_n(4, 4, is_lower_hex_digit),
            char('-'),
            take_while_m_n(4, 4, is_lower_hex_digit),
            char('-'),
            take_while_m_n(4, 4, is_lower_hex_digit),
            char('-'),
            take_while_m_n(12, 12, is_lower_hex_digit),
        )))(input)
    }

    fn take_many0<I, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, I, E>
    where
        I: nom::Offset + nom::InputTake + Clone + PartialEq + std::fmt::Debug,
        F: nom::Parser<I, I, E>,
        E: nom::error::ParseError<I>,
    {
        move |i: I| {
            let mut current = i.clone();
            loop {
                match f.parse(current.clone()) {
                    Err(nom::Err::Error(_)) => {
                        let offset = i.offset(&current);
                        let (after, before) = i.take_split(offset);
                        return Ok((after, before));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                    Ok((next, _)) => {
                        if next == current {
                            return Err(nom::Err::Error(E::from_error_kind(
                                i,
                                nom::error::ErrorKind::Many0,
                            )));
                        }

                        current = next;
                    }
                }
            }
        }
    }

    fn is_dec_digit_with_sep(i: impl AsChar + Copy) -> bool {
        i.is_dec_digit() || is_digit_sep(i.as_char())
    }

    fn is_hex_digit_with_sep(i: impl AsChar + Copy) -> bool {
        i.is_hex_digit() || is_digit_sep(i.as_char())
    }

    fn is_lower_hex_digit(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        ('a'..='f').contains(&c) || ('0'..='9').contains(&c)
    }

    fn is_xid_continue(i: impl AsChar + Copy) -> bool {
        let c = i.as_char();
        unicode_xid::UnicodeXID::is_xid_continue(c)
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
        std::iter::from_fn(move || match next_identifier(input) {
            Ok((i, o)) => {
                input = i;
                debug_assert_ne!(o, "");
                Some(o)
            }
            _ => None,
        })
    }
}

mod ascii_parser {
    use super::parser::next_identifier;

    pub(crate) fn iter_identifiers(mut input: &[u8]) -> impl Iterator<Item = &str> {
        std::iter::from_fn(move || match next_identifier(input) {
            Ok((i, o)) => {
                input = i;
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
    pub fn new_unchecked(token: &'t str, case: Case, offset: usize) -> Self {
        Self {
            token,
            case,
            offset,
        }
    }

    pub fn token(&self) -> &'t str {
        self.token
    }

    pub fn case(&self) -> Case {
        self.case
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Split into individual Words.
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

    pub fn new_unchecked(token: &'t str, case: Case, offset: usize) -> Self {
        Self {
            token,
            case,
            offset,
        }
    }

    pub fn token(&self) -> &'t str {
        self.token
    }

    pub fn case(&self) -> Case {
        self.case
    }

    pub fn offset(&self) -> usize {
        self.offset
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
