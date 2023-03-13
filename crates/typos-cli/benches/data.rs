pub static EMPTY: &str = "";

pub static NO_TOKENS: &str = "                    ";

pub static SINGLE_TOKEN: &str = "success";

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub static SHERLOCK: &str = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
";

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub static CODE: &str = "\
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

pub static CORPUS: &str = include_str!("../../typos-dict/assets/words.csv");

pub static DATA: &[(&str, &str)] = &[
    ("empty", EMPTY),
    ("no_tokens", NO_TOKENS),
    ("single_token", SINGLE_TOKEN),
    ("sherlock", SHERLOCK),
    ("code", CODE),
    ("corpus", CORPUS),
];
