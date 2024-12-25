#![allow(clippy::wildcard_imports)]

const MISS: &str = "finalizes";
const HIT: &str = "finallizes";

mod trie {
    use super::*;

    #[divan::bench(args = [unicase::UniCase::new(MISS)])]
    fn miss(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        typos_dict::WORD_TRIE.find(&word)
    }

    #[divan::bench(args = [unicase::UniCase::new(HIT)])]
    fn hit(word: unicase::UniCase<&str>) -> Option<&'static &[&str]> {
        typos_dict::WORD_TRIE.find(&word)
    }
}

fn main() {
    divan::main();
}
