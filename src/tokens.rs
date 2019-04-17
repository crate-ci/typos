#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'t> {
    pub token: &'t [u8],
    pub offset: usize,
}

impl<'t> Token<'t> {
    pub fn new(token: &'t [u8], offset: usize) -> Self {
        Self {
            token,
            offset,
        }
    }
}

pub fn tokenize(content: &[u8]) -> impl Iterator<Item=Token> {
    lazy_static::lazy_static! {
        static ref SPLIT: regex::bytes::Regex = regex::bytes::Regex::new(r#"\b(\p{Alphabetic}|\d|_)+\b"#).unwrap();
    }
    SPLIT.find_iter(content).map(|m| Token::new(m.as_bytes(), m.start()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_empty_is_empty() {
        let input = b"";
        let expected: Vec<Token> = vec![];
        let actual: Vec<_> = tokenize(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_word_is_word() {
        let input = b"word";
        let expected: Vec<Token> = vec![Token::new(b"word", 0)];
        let actual: Vec<_> = tokenize(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_space_separated_words() {
        let input = b"A B";
        let expected: Vec<Token> = vec![Token::new(b"A", 0), Token::new(b"B", 2)];
        let actual: Vec<_> = tokenize(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_dot_separated_words() {
        let input = b"A.B";
        let expected: Vec<Token> = vec![Token::new(b"A", 0), Token::new(b"B", 2)];
        let actual: Vec<_> = tokenize(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_namespace_separated_words() {
        let input = b"A::B";
        let expected: Vec<Token> = vec![Token::new(b"A", 0), Token::new(b"B", 3)];
        let actual: Vec<_> = tokenize(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let input = b"A_B";
        let expected: Vec<Token> = vec![Token::new(b"A_B", 0)];
        let actual: Vec<_> = tokenize(input).collect();
        assert_eq!(expected, actual);
    }
}
