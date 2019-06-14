pub const EMPTY: &str = "";

pub const NO_TOKENS: &str = "                    ";

pub const SINGLE_TOKEN: &str = "success";

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub const SHERLOCK: &'static str = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
";

// Stolen from https://github.com/BurntSushi/ripgrep/blob/master/grep-searcher/src/searcher/glue.rs
pub const CODE: &'static str = "\
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

pub const CORPUS: &str = include_str!("../assets/words.csv");
