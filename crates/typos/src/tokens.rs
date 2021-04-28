/// Define rules for tokenizaing a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenizerBuilder {
    ignore_hex: bool,
    leading_digits: bool,
}

impl TokenizerBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// Specify that hexadecimal numbers should be ignored.
    pub fn ignore_hex(&mut self, yes: bool) -> &mut Self {
        self.ignore_hex = yes;
        self
    }

    /// Specify that leading digits are allowed for Identifiers.
    pub fn leading_digits(&mut self, yes: bool) -> &mut Self {
        self.leading_digits = yes;
        self
    }

    pub fn build(&self) -> Tokenizer {
        let TokenizerBuilder {
            leading_digits,
            ignore_hex,
        } = self.clone();
        Tokenizer {
            leading_digits,
            ignore_hex,
        }
    }
}

impl Default for TokenizerBuilder {
    fn default() -> Self {
        Self {
            leading_digits: false,
            ignore_hex: true,
        }
    }
}

/// Extract Identifiers from a buffer.
#[derive(Debug, Clone)]
pub struct Tokenizer {
    leading_digits: bool,
    ignore_hex: bool,
}

impl Tokenizer {
    pub fn new() -> Self {
        TokenizerBuilder::default().build()
    }

    pub fn parse_str<'c>(&'c self, content: &'c str) -> impl Iterator<Item = Identifier<'c>> {
        parser::iter_literals(content).filter_map(move |identifier| {
            let offset = offset(content.as_bytes(), identifier.as_bytes());
            self.transform(identifier, offset)
        })
    }

    pub fn parse_bytes<'c>(&'c self, content: &'c [u8]) -> impl Iterator<Item = Identifier<'c>> {
        Utf8Chunks::new(content).flat_map(move |c| {
            let chunk_offset = offset(content, c.as_bytes());
            self.parse_str(c).map(move |i| {
                Identifier::new_unchecked(i.token(), i.case(), i.offset() + chunk_offset)
            })
        })
    }

    fn transform<'i>(&self, identifier: &'i str, offset: usize) -> Option<Identifier<'i>> {
        debug_assert!(!identifier.is_empty());
        if self.leading_digits {
            if is_number(identifier.as_bytes()) {
                return None;
            }

            if self.ignore_hex && is_hex(identifier.as_bytes()) {
                return None;
            }
        } else if is_digit(identifier.as_bytes()[0]) {
            return None;
        }

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
        loop {
            if self.source.is_empty() {
                return None;
            }
            match simdutf8::compat::from_utf8(self.source) {
                Ok(valid) => {
                    self.source = b"";
                    return Some(valid);
                }
                Err(error) => {
                    let (valid, after_valid) = self.source.split_at(error.valid_up_to());

                    if let Some(invalid_sequence_length) = error.error_len() {
                        self.source = &after_valid[invalid_sequence_length..];
                    } else {
                        self.source = b"";
                    }

                    let valid = unsafe { std::str::from_utf8_unchecked(valid) };
                    return Some(valid);
                }
            }
        }
    }
}

fn is_number(ident: &[u8]) -> bool {
    ident.iter().all(|b| is_digit(*b) || is_digit_sep(*b))
}

fn is_hex(ident: &[u8]) -> bool {
    if ident.len() < 3 {
        false
    } else {
        ident[0] == b'0'
            && ident[1] == b'x'
            && ident[2..]
                .iter()
                .all(|b| is_hex_digit(*b) || is_digit_sep(*b))
    }
}

#[inline]
fn is_digit(chr: u8) -> bool {
    chr.is_ascii_digit()
}

#[inline]
fn is_digit_sep(chr: u8) -> bool {
    // `_`: number literal separator in Rust and other languages
    // `'`: number literal separator in C++
    chr == b'_' || chr == b'\''
}

#[inline]
fn is_hex_digit(chr: u8) -> bool {
    chr.is_ascii_hexdigit()
}

mod parser {
    use nom::bytes::complete::*;
    use nom::sequence::*;
    use nom::IResult;

    pub(crate) fn iter_literals(mut input: &str) -> impl Iterator<Item = &str> {
        std::iter::from_fn(move || match next_literal(input) {
            Ok((i, o)) => {
                input = i;
                assert_ne!(o, "");
                Some(o)
            }
            _ => None,
        })
    }

    fn next_literal(input: &str) -> IResult<&str, &str> {
        preceded(literal_sep, identifier)(input)
    }

    fn literal_sep(input: &str) -> IResult<&str, &str> {
        take_till(|c: char| unicode_xid::UnicodeXID::is_xid_continue(c))(input)
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        // Generally a language would be `{XID_Start}{XID_Continue}*` but going with only
        // `{XID_Continue}+` because XID_Continue is a superset of XID_Start and rather catch odd
        // or unexpected cases than strip off start characters to a word since we aren't doing a
        // proper word boundary parse
        take_while1(|c: char| unicode_xid::UnicodeXID::is_xid_continue(c))(input)
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
                assert!(self.start_mode == WordMode::Boundary);
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
    fn tokenize_ignore_hex_enabled() {
        let parser = TokenizerBuilder::new()
            .ignore_hex(true)
            .leading_digits(true)
            .build();

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
    fn tokenize_ignore_hex_disabled() {
        let parser = TokenizerBuilder::new()
            .ignore_hex(false)
            .leading_digits(true)
            .build();

        let input = "Hello 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("0xDEADBEEF", Case::None, 6),
            Identifier::new_unchecked("World", Case::None, 17),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_leading_digits_enabled() {
        let parser = TokenizerBuilder::new()
            .ignore_hex(false)
            .leading_digits(true)
            .build();

        let input = "Hello 0Hello 124 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
            Identifier::new_unchecked("0Hello", Case::None, 6),
            Identifier::new_unchecked("0xDEADBEEF", Case::None, 17),
            Identifier::new_unchecked("World", Case::None, 28),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_leading_digits_disabled() {
        let parser = TokenizerBuilder::new()
            .ignore_hex(false)
            .leading_digits(false)
            .build();

        let input = "Hello 0Hello 124 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", Case::None, 0),
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
