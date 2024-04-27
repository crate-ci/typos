#![allow(dead_code)]

pub(crate) static EMPTY: &str = "";

pub(crate) static NO_TOKENS: &str = "                    ";

pub(crate) static SINGLE_TOKEN: &str = "success";

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub(crate) static SHERLOCK: &str = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
";

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub(crate) static CODE: &str = "\
extern crate snap;
use std::io;
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    // Wrap the stdin reader in a Snappy reader.
    let mut rdr = snap::Reader::new(stdin.lock());
    let mut wtr = stdout.lock();
    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");
}
";

pub(crate) static CORPUS: &str = include_str!("../../typos-dict/assets/words.csv");

#[derive(Debug)]
pub(crate) struct Data(&'static str, &'static str);

impl Data {
    pub(crate) const fn name(&self) -> &'static str {
        self.0
    }

    pub(crate) const fn content(&self) -> &'static str {
        self.1
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

pub(crate) static DATA: &[Data] = &[
    Data("empty", EMPTY),
    Data("no_tokens", NO_TOKENS),
    Data("single_token", SINGLE_TOKEN),
    Data("sherlock", SHERLOCK),
    Data("code", CODE),
    Data("corpus", CORPUS),
];
