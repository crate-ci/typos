pub static EMPTY: &str = include_str!("data/empty.txt");

pub static NO_TOKENS: &str = include_str!("data/no_tokens.txt");

pub static SINGLE_TOKEN: &str = include_str!("data/single_token.txt");

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub static SHERLOCK: &str = include_str!("data/sherlock.txt");

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub static CODE: &str = include_str!("data/code.txt");

pub static CORPUS: &str = include_str!("../../typos-dict/assets/words.csv");

pub static DATA: &[(&str, &str)] = &[
    ("empty", EMPTY),
    ("no_tokens", NO_TOKENS),
    ("single_token", SINGLE_TOKEN),
    ("sherlock", SHERLOCK),
    ("code", CODE),
    ("corpus", CORPUS),
];
