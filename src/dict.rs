include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub struct Dictionary {
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary { }
    }

    pub fn correct_str<'s>(&'s self, word: &str) -> Option<&'s str> {
        DICTIONARY.get(word).map(|s| *s)
    }

    pub fn correct_bytes<'s>(&'s self, word: &[u8]) -> Option<&'s [u8]> {
        std::str::from_utf8(word).ok().and_then(|word| DICTIONARY.get(word)).map(|s| s.as_bytes())
    }
}
