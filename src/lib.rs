use std::fs::File;
use std::io::Read;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub struct Token<'t> {
    pub token: &'t [u8],
    pub offset: usize,
}

pub fn tokenize(content: &[u8]) -> impl Iterator<Item=Token> {
    lazy_static::lazy_static! {
        static ref SPLIT: regex::bytes::Regex = regex::bytes::Regex::new(r#"\b\w+\b"#).unwrap();
    }
    SPLIT.find_iter(content).map(|m| {
        Token {
            token: m.as_bytes(),
            offset: m.start(),
        }
    })
}

pub fn process_file(path: &std::path::Path, dictionary: &Corrections) -> Result<(), failure::Error> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    for (line_idx, line) in grep_searcher::LineIter::new(b'\n', &buffer).enumerate() {
        let line_num = line_idx + 1;
        for token in tokenize(line) {
            if let Some(word) = std::str::from_utf8(token.token).ok() {
                if let Some(correction) = dictionary.correct_str(word) {
                    let column = token.offset;
                    println!("{}:{}:{}: {} -> {}", path.display(), line_num, column, word, correction);
                }
            }
        }
    }

    Ok(())
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

