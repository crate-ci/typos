#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Title,
    Lower,
    Scream,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'t> {
    token: &'t str,
    offset: usize,
}

impl<'t> Identifier<'t> {
    pub fn new(token: &'t str, offset: usize) -> Result<Self, failure::Error> {
        let mut itr = Self::parse(token.as_bytes());
        let mut item = itr
            .next()
            .ok_or_else(|| failure::format_err!("Invalid ident (none found): {:?}", token))?;
        if item.offset != 0 {
            return Err(failure::format_err!(
                "Invalid ident (padding found): {:?}",
                token
            ));
        }
        item.offset += offset;
        if itr.next().is_some() {
            return Err(failure::format_err!(
                "Invalid ident (contains more than one): {:?}",
                token
            ));
        }
        Ok(item)
    }

    pub(crate) fn new_unchecked(token: &'t str, offset: usize) -> Self {
        Self { token, offset }
    }

    pub fn parse(content: &[u8]) -> impl Iterator<Item = Identifier<'_>> {
        lazy_static::lazy_static! {
            // Getting false positives for this lint
            #[allow(clippy::invalid_regex)]
            static ref SPLIT: regex::bytes::Regex = regex::bytes::Regex::new(r#"\b(\p{Alphabetic}|\d|_)+\b"#).unwrap();
        }
        SPLIT.find_iter(content).filter_map(|m| {
            let s = std::str::from_utf8(m.as_bytes()).ok();
            s.map(|s| Identifier::new_unchecked(s, m.start()))
        })
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
        Identifier::new(token, offset)?;
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

    pub(crate) fn new_unchecked(token: &'t str, case: Case, offset: usize) -> Self {
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
            if start == i {
                start += 1;
            }
            continue;
        }
        if start_mode == WordMode::Boundary {
            start_mode = cur_mode;
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
        let input = b"";
        let expected: Vec<Identifier> = vec![];
        let actual: Vec<_> = Identifier::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_word_is_word() {
        let input = b"word";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("word", 0)];
        let actual: Vec<_> = Identifier::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_space_separated_words() {
        let input = b"A B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", 0),
            Identifier::new_unchecked("B", 2),
        ];
        let actual: Vec<_> = Identifier::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_dot_separated_words() {
        let input = b"A.B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", 0),
            Identifier::new_unchecked("B", 2),
        ];
        let actual: Vec<_> = Identifier::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_namespace_separated_words() {
        let input = b"A::B";
        let expected: Vec<Identifier> = vec![
            Identifier::new_unchecked("A", 0),
            Identifier::new_unchecked("B", 3),
        ];
        let actual: Vec<_> = Identifier::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let input = b"A_B";
        let expected: Vec<Identifier> = vec![Identifier::new_unchecked("A_B", 0)];
        let actual: Vec<_> = Identifier::parse(input).collect();
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
            let ident = Identifier::new(input, 0).unwrap();
            let result: Vec<_> = ident.split().map(|w| (w.token, w.case, w.offset)).collect();
            assert_eq!(&result, expected);
        }
    }
}
