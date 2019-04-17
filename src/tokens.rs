#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol<'t> {
    pub token: &'t [u8],
    pub offset: usize,
}

impl<'t> Symbol<'t> {
    pub fn new(token: &'t [u8], offset: usize) -> Self {
        Self {
            token,
            offset,
        }
    }

    pub fn parse<'s>(content: &'s [u8]) -> impl Iterator<Item=Symbol<'s>> {
        lazy_static::lazy_static! {
            static ref SPLIT: regex::bytes::Regex = regex::bytes::Regex::new(r#"\b(\p{Alphabetic}|\d|_)+\b"#).unwrap();
        }
        SPLIT.find_iter(content).map(|m| Symbol::new(m.as_bytes(), m.start()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_empty_is_empty() {
        let input = b"";
        let expected: Vec<Symbol> = vec![];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_word_is_word() {
        let input = b"word";
        let expected: Vec<Symbol> = vec![Symbol::new(b"word", 0)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_space_separated_words() {
        let input = b"A B";
        let expected: Vec<Symbol> = vec![Symbol::new(b"A", 0), Symbol::new(b"B", 2)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_dot_separated_words() {
        let input = b"A.B";
        let expected: Vec<Symbol> = vec![Symbol::new(b"A", 0), Symbol::new(b"B", 2)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_namespace_separated_words() {
        let input = b"A::B";
        let expected: Vec<Symbol> = vec![Symbol::new(b"A", 0), Symbol::new(b"B", 3)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let input = b"A_B";
        let expected: Vec<Symbol> = vec![Symbol::new(b"A_B", 0)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }
}
