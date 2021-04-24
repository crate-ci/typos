/// Define rules for tokenizaing a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenizerBuilder {
    ignore_hex: bool,
    leading_digits: bool,
    leading_chars: String,
    include_digits: bool,
    include_chars: String,
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

    /// Extend accepted leading characters for Identifiers.
    pub fn leading_chars(&mut self, chars: String) -> &mut Self {
        self.leading_chars = chars;
        self
    }

    /// Specify that digits can be included in Identifiers.
    pub fn include_digits(&mut self, yes: bool) -> &mut Self {
        self.include_digits = yes;
        self
    }

    /// Extend accepted characters for Identifiers.
    pub fn include_chars(&mut self, chars: String) -> &mut Self {
        self.include_chars = chars;
        self
    }

    pub fn build(&self) -> Tokenizer {
        let mut pattern = r#"\b("#.to_owned();
        Self::push_pattern(&mut pattern, self.leading_digits, &self.leading_chars);
        Self::push_pattern(&mut pattern, self.include_digits, &self.include_chars);
        pattern.push_str(r#"*)\b"#);

        let words_str = regex::Regex::new(&pattern).unwrap();

        Tokenizer {
            words_str,
            // `leading_digits` let's us bypass the regexes since you can't have a decimal or
            // hexadecimal number without a leading digit.
            ignore_numbers: self.leading_digits,
            ignore_hex: self.ignore_hex && self.leading_digits,
        }
    }

    fn push_pattern(pattern: &mut String, digits: bool, chars: &str) {
        pattern.push_str(r#"(\p{Alphabetic}"#);
        if digits {
            pattern.push_str(r#"|\d"#);
        }
        for grapheme in unicode_segmentation::UnicodeSegmentation::graphemes(chars, true) {
            let escaped = regex::escape(&grapheme);
            pattern.push_str(&format!("|{}", escaped));
        }
        pattern.push(')');
    }
}

impl Default for TokenizerBuilder {
    fn default() -> Self {
        Self {
            ignore_hex: true,
            leading_digits: false,
            leading_chars: "_".to_owned(),
            include_digits: true,
            include_chars: "_'".to_owned(),
        }
    }
}

/// Extract Identifiers from a buffer.
#[derive(Debug, Clone)]
pub struct Tokenizer {
    words_str: regex::Regex,
    ignore_numbers: bool,
    ignore_hex: bool,
}

impl Tokenizer {
    pub fn new() -> Self {
        TokenizerBuilder::default().build()
    }

    pub fn parse_str<'c>(&'c self, content: &'c str) -> impl Iterator<Item = Identifier<'c>> {
        self.words_str
            .find_iter(content)
            .filter(move |m| self.accept(m.as_str()))
            .map(|m| Identifier::new_unchecked(m.as_str(), Case::None, m.start()))
    }

    pub fn parse_bytes<'c>(&'c self, content: &'c [u8]) -> impl Iterator<Item = Identifier<'c>> {
        Utf8Chunks::new(content).flat_map(move |c| {
            let chunk_offset = offset(content, c.as_bytes());
            self.parse_str(c).map(move |i| {
                Identifier::new_unchecked(i.token(), i.case(), i.offset() + chunk_offset)
            })
        })
    }

    fn accept(&self, contents: &str) -> bool {
        if self.ignore_numbers && is_number(contents) {
            return false;
        }

        if self.ignore_hex && is_hex(contents) {
            return false;
        }

        true
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
            match std::str::from_utf8(self.source) {
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

// `_`: number literal separator in Rust and other languages
// `'`: number literal separator in C++
static DIGITS: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r#"^[0-9_']+$"#).unwrap());

fn is_number(ident: &str) -> bool {
    DIGITS.is_match(ident)
}

// `_`: number literal separator in Rust and other languages
// `'`: number literal separator in C++
static HEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r#"^0[xX][0-9a-fA-F_']+$"#).unwrap());

fn is_hex(ident: &str) -> bool {
    HEX.is_match(ident)
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
        let parser = TokenizerBuilder::new().ignore_hex(true).build();

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
