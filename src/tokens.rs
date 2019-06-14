#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol<'t> {
    pub token: &'t str,
    pub offset: usize,
}

impl<'t> Symbol<'t> {
    pub fn new(token: &'t str, offset: usize) -> Self {
        Self { token, offset }
    }

    pub fn parse(content: &[u8]) -> impl Iterator<Item = Symbol<'_>> {
        lazy_static::lazy_static! {
            // Getting false positives for this lint
            #[allow(clippy::invalid_regex)]
            static ref SPLIT: regex::bytes::Regex = regex::bytes::Regex::new(r#"\b(\p{Alphabetic}|\d|_)+\b"#).unwrap();
        }
        SPLIT.find_iter(content).filter_map(|m| {
            let s = std::str::from_utf8(m.as_bytes()).ok();
            s.map(|s| Symbol::new(s, m.start()))
        })
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
        let expected: Vec<Symbol> = vec![Symbol::new("word", 0)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_space_separated_words() {
        let input = b"A B";
        let expected: Vec<Symbol> = vec![Symbol::new("A", 0), Symbol::new("B", 2)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_dot_separated_words() {
        let input = b"A.B";
        let expected: Vec<Symbol> = vec![Symbol::new("A", 0), Symbol::new("B", 2)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_namespace_separated_words() {
        let input = b"A::B";
        let expected: Vec<Symbol> = vec![Symbol::new("A", 0), Symbol::new("B", 3)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenize_underscore_doesnt_separate() {
        let input = b"A_B";
        let expected: Vec<Symbol> = vec![Symbol::new("A_B", 0)];
        let actual: Vec<_> = Symbol::parse(input).collect();
        assert_eq!(expected, actual);
    }
}
