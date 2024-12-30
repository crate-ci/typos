#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

mod cased_map_codegen;
mod map_codegen;
mod ordered_map_codegen;
mod trie_codegen;

mod miss {
    use super::*;

    const MISS: &str = "finalizes";

    #[divan::bench(args = [MISS])]
    fn cased_map(word: &str) -> Option<&'static &[&str]> {
        cased_map_codegen::WORD_ASCII_LOWER.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn map(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        map_codegen::WORD.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn trie(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        trie_codegen::WORD.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn ordered_map(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        ordered_map_codegen::WORD.find(&word)
    }
}

mod hit {
    use super::*;

    const HIT: &str = "finallizes";

    #[divan::bench(args = [HIT])]
    fn cased_map(word: &str) -> Option<&'static &[&str]> {
        cased_map_codegen::WORD_ASCII_LOWER.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn map(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        map_codegen::WORD.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn trie(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        trie_codegen::WORD.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn ordered_map(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        ordered_map_codegen::WORD.find(&word)
    }
}

fn main() {
    divan::main();
}
