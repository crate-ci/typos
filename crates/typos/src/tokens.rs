#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserBuilder {
    ignore_hex: bool,
    leading_digits: bool,
    leading_chars: String,
    include_digits: bool,
    include_chars: String,
}

impl ParserBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ignore_hex(&mut self, yes: bool) -> &mut Self {
        self.ignore_hex = yes;
        self
    }

    pub fn leading_digits(&mut self, yes: bool) -> &mut Self {
        self.leading_digits = yes;
        self
    }

    pub fn leading_chars(&mut self, chars: String) -> &mut Self {
        self.leading_chars = chars;
        self
    }

    pub fn include_digits(&mut self, yes: bool) -> &mut Self {
        self.include_digits = yes;
        self
    }

    pub fn include_chars(&mut self, chars: String) -> &mut Self {
        self.include_chars = chars;
        self
    }

    pub fn build(&self) -> Parser {
        let mut pattern = r#"\b("#.to_owned();
        Self::push_pattern(&mut pattern, self.leading_digits, &self.leading_chars);
        Self::push_pattern(&mut pattern, self.include_digits, &self.include_chars);
        pattern.push_str(r#"*)\b"#);

        let words_str = regex::Regex::new(&pattern).unwrap();
        let words_bytes = regex::bytes::Regex::new(&pattern).unwrap();

        Parser {
            words_str,
            words_bytes,
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

impl Default for ParserBuilder {
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

#[derive(Debug, Clone)]
pub struct Parser {
    words_str: regex::Regex,
    words_bytes: regex::bytes::Regex,
    ignore_numbers: bool,
    ignore_hex: bool,
}

impl Parser {
    pub fn new() -> Self {
        ParserBuilder::default().build()
    }

    pub fn parse_str<'c>(&'c self, content: &'c str) -> impl Iterator<Item = Identifier<'c>> {
        self.words_str
            .find_iter(content)
            .filter(move |m| self.accept(m.as_str().as_bytes()))
            .map(|m| Identifier::new_unchecked(m.as_str(), m.start()))
    }

    pub fn parse_bytes<'c>(&'c self, content: &'c [u8]) -> impl Iterator<Item = Identifier<'c>> {
        self.words_bytes
            .find_iter(content)
            .filter(move |m| self.accept(m.as_bytes()))
            .filter_map(|m| {
                let s = std::str::from_utf8(m.as_bytes()).ok();
                s.map(|s| Identifier::new_unchecked(s, m.start()))
            })
    }

    fn accept(&self, contents: &[u8]) -> bool {
        if self.ignore_numbers && is_number(contents) {
            return false;
        }

        if self.ignore_hex && is_hex(contents) {
            return false;
        }

        true
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

// `_`: number literal separator in Rust and other languages
// `'`: number literal separator in C++
static DIGITS: once_cell::sync::Lazy<regex::bytes::Regex> =
    once_cell::sync::Lazy::new(|| regex::bytes::Regex::new(r#"^[0-9_']+$"#).unwrap());

fn is_number(ident: &[u8]) -> bool {
    DIGITS.is_match(ident)
}

// `_`: number literal separator in Rust and other languages
// `'`: number literal separator in C++
static HEX: once_cell::sync::Lazy<regex::bytes::Regex> =
    once_cell::sync::Lazy::new(|| regex::bytes::Regex::new(r#"^0[xX][0-9a-fA-F_']+$"#).unwrap());

fn is_hex(ident: &[u8]) -> bool {
    HEX.is_match(ident)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'t> {
    token: &'t str,
    offset: usize,
}

impl<'t> Identifier<'t> {
    pub fn new_unchecked(token: &'t str, offset: usize) -> Self {
        Self { token, offset }
    }

    pub fn token(&self) -> &'t str {
        self.token
    }

    pub fn case(&self) -> Case {
        Case::None
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn split(&self) -> impl Iterator<Item = Word<'t>> {
        split_ident(self.token, self.offset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word<'t> {
    token: &'t str,
    case: Case,
    offset: usize,
}

impl<'t> Word<'t> {
    pub fn new(token: &'t str, offset: usize) -> Result<Self, std::io::Error> {
        let mut itr = split_ident(token, 0);
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

fn split_ident(ident: &str, offset: usize) -> impl Iterator<Item = Word<'_>> {
    SplitIdent::new(ident, offset)
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
                            Case::Scream,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Title,
    Lower,
    Scream,
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
            (WordMode::Uppercase, WordMode::Uppercase) => Case::Scream,
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
        let parser = Parser::new();

        let input = "";
        let expected: Vec<Identifier> = vec![];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_word_is_word() {
        let parser = Parser::new();

        let input = "word";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("word", 0)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_space_separated_words() {
        let parser = Parser::new();

        let input = "A B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", 0),
            Identifier::new_unchecked("B", 2),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_dot_separated_words() {
        let parser = Parser::new();

        let input = "A.B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", 0),
            Identifier::new_unchecked("B", 2),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_namespace_separated_words() {
        let parser = Parser::new();

        let input = "A::B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", 0),
            Identifier::new_unchecked("B", 3),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let parser = Parser::new();

        let input = "A_B";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("A_B", 0)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_hex_enabled() {
        let parser = ParserBuilder::new().ignore_hex(true).build();

        let input = "Hello 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", 0),
            Identifier::new_unchecked("World", 17),
        ];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse_str(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_ignore_hex_disabled() {
        let parser = ParserBuilder::new()
            .ignore_hex(false)
            .leading_digits(true)
            .build();

        let input = "Hello 0xDEADBEEF World";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("Hello", 0),
            Identifier::new_unchecked("0xDEADBEEF", 6),
            Identifier::new_unchecked("World", 17),
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
            ("MyC", &[("My", Case::Title, 0), ("C", Case::Scream, 2)]),
            ("HTML", &[("HTML", Case::Scream, 0)]),
            (
                "PDFLoader",
                &[("PDF", Case::Scream, 0), ("Loader", Case::Title, 3)],
            ),
            (
                "AString",
                &[("A", Case::Scream, 0), ("String", Case::Title, 1)],
            ),
            (
                "SimpleXMLParser",
                &[
                    ("Simple", Case::Title, 0),
                    ("XML", Case::Scream, 6),
                    ("Parser", Case::Title, 9),
                ],
            ),
            (
                "vimRPCPlugin",
                &[
                    ("vim", Case::Lower, 0),
                    ("RPC", Case::Scream, 3),
                    ("Plugin", Case::Title, 6),
                ],
            ),
            (
                "GL11Version",
                &[
                    ("GL", Case::Scream, 0),
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
                &[("BFG", Case::Scream, 0), ("9000", Case::None, 3)],
            ),
        ];
        for (input, expected) in cases.iter() {
            let ident = Identifier::new_unchecked(input, 0);
            let result: Vec<_> = ident.split().map(|w| (w.token, w.case, w.offset)).collect();
            assert_eq!(&result, expected);
        }
    }
}
