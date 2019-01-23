#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Read;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

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
        static ref SPLIT: regex::bytes::Regex = regex::bytes::Regex::new(r#"\b\w+\b"#).unwrap();
    }
    SPLIT.find_iter(content).map(|m| Token::new(m.as_bytes(), m.start()))
}

#[derive(Debug, Serialize)]
pub struct Message<'m> {
    path: &'m std::path::Path,
    #[serde(skip)]
    line: &'m [u8],
    line_num: usize,
    col_num: usize,
    word: &'m str,
    correction: &'m str,
}

pub fn process_file(path: &std::path::Path, dictionary: &Corrections, report: Report) -> Result<(), failure::Error> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    for (line_idx, line) in grep_searcher::LineIter::new(b'\n', &buffer).enumerate() {
        let line_num = line_idx + 1;
        for token in tokenize(line) {
            if let Some(word) = std::str::from_utf8(token.token).ok() {
                if let Some(correction) = dictionary.correct_str(word) {
                    let col_num = token.offset;
                    let msg = Message {
                        path,
                        line,
                        line_num,
                        col_num,
                        word,
                        correction,
                    };
                    report(msg);
                }
            }
        }
    }

    Ok(())
}

pub type Report = fn(msg: Message);

pub fn print_silent(_: Message) {
}

pub fn print_brief(msg: Message) {
    println!("{}:{}:{}: {} -> {}", msg.path.display(), msg.line_num, msg.col_num, msg.word, msg.correction);
}

pub fn print_long(msg: Message) {
    let line_num = msg.line_num.to_string();
    let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

    let hl_indent: String = itertools::repeat_n(" ", msg.col_num).collect();
    let hl: String = itertools::repeat_n("^", msg.word.len()).collect();

    println!("error: `{}` should be `{}`", msg.word, msg.correction);
    println!("  --> {}:{}:{}", msg.path.display(), msg.line_num, msg.col_num);
    println!("{} |", line_indent);
    println!("{} | {}", msg.line_num, String::from_utf8_lossy(msg.line).trim_end());
    println!("{} | {}{}", line_indent, hl_indent, hl);
    println!("{} |", line_indent);
}

pub fn print_json(msg: Message) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}

pub struct Corrections {
}

impl Corrections {
    pub fn new() -> Self {
        Corrections { }
    }

    pub fn correct_str<'s>(&'s self, word: &str) -> Option<&'s str> {
        DICTIONARY.get(word).map(|s| *s)
    }

    pub fn correct_bytes<'s>(&'s self, word: &[u8]) -> Option<&'s [u8]> {
        std::str::from_utf8(word).ok().and_then(|word| DICTIONARY.get(word)).map(|s| s.as_bytes())
    }
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
