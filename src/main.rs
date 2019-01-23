// 2015-edition macros.
#[macro_use]
extern crate clap;

use structopt::StructOpt;

arg_enum!{
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Format {
        Silent,
        Brief,
        Long,
        Json,
    }
}

impl Format {
    fn report(self) -> scorrect::Report {
        match self {
            Format::Silent => scorrect::print_silent,
            Format::Brief => scorrect::print_brief,
            Format::Long => scorrect::print_long,
            Format::Json => scorrect::print_json,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Long
    }
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    /// Paths to check
    path: Vec<std::path::PathBuf>,


    #[structopt(long = "format",
                raw(possible_values = "&Format::variants()", case_insensitive = "true"),
                default_value = "long")]
    pub format: Format,

    #[structopt(short="j", long="threads", default_value="0")]
    /// The approximate number of threads to use.
    threads: usize,
}

impl Options {
    pub fn infer(mut self) -> Self {
        if self.path.len() == 1 {
            if self.path[0].is_file() {
                self.threads = 1;
            }
        }

        self
    }
}

fn run() -> Result<(), failure::Error> {
    let options = Options::from_args().infer();

    let dictionary = scorrect::Corrections::new();

    let first_path = &options.path.get(0).expect("arg parsing enforces at least one");
    let mut walk = ignore::WalkBuilder::new(first_path);
    for path in &options.path[1..] {
        walk.add(path);
    }
    walk.threads(options.threads);
    // TODO Add build_parallel for options.threads != 1
    for entry in walk.build() {
        let entry = entry?;
        if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
            scorrect::process_file(entry.path(), &dictionary, options.format.report())?;
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
