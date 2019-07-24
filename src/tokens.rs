#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Title,
    Lower,
    Scream,
    None,
}

#[derive(Debug, Clone, Default)]
pub struct ParserBuilder {}

impl ParserBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Parser {
        let pattern = r#"\b(\p{Alphabetic}|\d|_|')+\b"#;
        let words_str = regex::Regex::new(pattern).unwrap();
        let words_bytes = regex::bytes::Regex::new(pattern).unwrap();
        Parser {
            words_str,
            words_bytes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    words_str: regex::Regex,
    words_bytes: regex::bytes::Regex,
}

impl Parser {
    pub fn new() -> Self {
        ParserBuilder::default().build()
    }

    pub fn parse<'c>(&'c self, content: &'c str) -> impl Iterator<Item = Identifier<'c>> {
        self.words_str
            .find_iter(content)
            .map(|m| Identifier::new_unchecked(m.as_str(), m.start()))
    }

    pub fn parse_bytes<'c>(&'c self, content: &'c [u8]) -> impl Iterator<Item = Identifier<'c>> {
        self.words_bytes.find_iter(content).filter_map(|m| {
            let s = std::str::from_utf8(m.as_bytes()).ok();
            s.map(|s| Identifier::new_unchecked(s, m.start()))
        })
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn token(&self) -> &str {
        self.token
    }

    pub fn case(&self) -> Case {
        Case::None
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn split(&self) -> impl Iterator<Item = Word<'_>> {
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
    pub fn new(token: &'t str, offset: usize) -> Result<Self, failure::Error> {
        let mut itr = split_ident(token, 0);
        let mut item = itr
            .next()
            .ok_or_else(|| failure::format_err!("Invalid word (none found): {:?}", token))?;
        if item.offset != 0 {
            return Err(failure::format_err!(
                "Invalid word (padding found): {:?}",
                token
            ));
        }
        item.offset += offset;
        if itr.next().is_some() {
            return Err(failure::format_err!(
                "Invalid word (contains more than one): {:?}",
                token
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

    pub fn token(&self) -> &str {
        self.token
    }

    pub fn case(&self) -> Case {
        self.case
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
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

fn split_ident(ident: &str, offset: usize) -> impl Iterator<Item = Word<'_>> {
    let mut result = vec![];

    let mut char_indices = ident.char_indices().peekable();
    let mut start = 0;
    let mut start_mode = WordMode::Boundary;
    let mut last_mode = WordMode::Boundary;
    while let Some((i, c)) = char_indices.next() {
        let cur_mode = WordMode::classify(c);
        if cur_mode == WordMode::Boundary {
            assert!(start_mode == WordMode::Boundary);
            continue;
        }
        if start_mode == WordMode::Boundary {
            start_mode = cur_mode;
            start = i;
        }

        if let Some(&(next_i, next)) = char_indices.peek() {
            // The mode including the current character, assuming the current character does
            // not result in a word boundary.
            let next_mode = WordMode::classify(next);

            match (last_mode, cur_mode, next_mode) {
                // cur_mode is last of current word
                (_, _, WordMode::Boundary)
                | (_, WordMode::Lowercase, WordMode::Number)
                | (_, WordMode::Uppercase, WordMode::Number)
                | (_, WordMode::Number, WordMode::Lowercase)
                | (_, WordMode::Number, WordMode::Uppercase)
                | (_, WordMode::Lowercase, WordMode::Uppercase) => {
                    let case = start_mode.case(cur_mode);
                    result.push(Word::new_unchecked(
                        &ident[start..next_i],
                        case,
                        start + offset,
                    ));
                    start = next_i;
                    start_mode = WordMode::Boundary;
                    last_mode = WordMode::Boundary;
                }
                // cur_mode is start of next word
                (WordMode::Uppercase, WordMode::Uppercase, WordMode::Lowercase) => {
                    result.push(Word::new_unchecked(
                        &ident[start..i],
                        Case::Scream,
                        start + offset,
                    ));
                    start = i;
                    start_mode = cur_mode;
                    last_mode = WordMode::Boundary;
                }
                // No word boundary
                (_, _, _) => {
                    last_mode = cur_mode;
                }
            }
        } else {
            // Collect trailing characters as a word
            let case = start_mode.case(cur_mode);
            result.push(Word::new_unchecked(&ident[start..], case, start + offset));
            break;
        }
    }

    result.into_iter()
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
        let actual: Vec<_> = parser.parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_word_is_word() {
        let parser = Parser::new();

        let input = "word";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("word", 0)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse(input).collect();
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
        let actual: Vec<_> = parser.parse(input).collect();
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
        let actual: Vec<_> = parser.parse(input).collect();
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
        let actual: Vec<_> = parser.parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let parser = Parser::new();

        let input = "A_B";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("A_B", 0)];
        let actual: Vec<_> = parser.parse_bytes(input.as_bytes()).collect();
        assert_eq!(expected, actual);
        let actual: Vec<_> = parser.parse(input).collect();
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
