#![allow(clippy::wildcard_imports)]

mod map_codegen;
mod table_codegen;
mod trie_codegen;

mod miss {
    use super::*;

    const MISS: &str = "finalizes";

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn map(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        map_codegen::WORD.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn trie(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        trie_codegen::WORD_TRIE.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn table(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        table_codegen::WORD.find(&word)
    }
}

mod hit {
    use super::*;

    const HIT: &str = "finallizes";

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn map(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        map_codegen::WORD.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn trie(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        trie_codegen::WORD_TRIE.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn table(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        table_codegen::WORD.find(&word)
    }
}

fn main() {
    divan::main();
}
