// 2015-edition macros.
#[macro_use]
extern crate clap;

use structopt::StructOpt;

arg_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Format {
        Silent,
        Brief,
        Long,
        Json,
    }
}

impl Format {
    fn report(self) -> typos::report::Report {
        match self {
            Format::Silent => typos::report::print_silent,
            Format::Brief => typos::report::print_brief,
            Format::Long => typos::report::print_long,
            Format::Json => typos::report::print_json,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Long
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(parse(from_os_str), default_value = ".")]
    /// Paths to check
    path: Vec<std::path::PathBuf>,

    #[structopt(
        long = "format",
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
        default_value = "long"
    )]
    pub format: Format,

    #[structopt(short = "j", long = "threads", default_value = "0")]
    /// The approximate number of threads to use.
    threads: usize,

    #[structopt(long, raw(overrides_with = r#""no-hidden""#))]
    /// Search hidden files and directories.
    hidden: bool,
    #[structopt(long, raw(overrides_with = r#""hidden""#), raw(hidden = "true"))]
    no_hidden: bool,

    #[structopt(long, raw(overrides_with = r#""ignore""#))]
    /// Don't respect ignore files.
    no_ignore: bool,
    #[structopt(long, raw(overrides_with = r#""no-ignore""#), raw(hidden = "true"))]
    ignore: bool,

    #[structopt(long, raw(overrides_with = r#""ignore-dot""#))]
    /// Don't respect .ignore files.
    no_ignore_dot: bool,
    #[structopt(long, raw(overrides_with = r#""no-ignore-dot""#), raw(hidden = "true"))]
    ignore_dot: bool,

    #[structopt(long, raw(overrides_with = r#""ignore-global""#))]
    /// Don't respect global ignore files.
    no_ignore_global: bool,
    #[structopt(
        long,
        raw(overrides_with = r#""no-ignore-global""#),
        raw(hidden = "true")
    )]
    ignore_global: bool,

    #[structopt(long, raw(overrides_with = r#""ignore-parent""#))]
    /// Don't respect ignore files in parent directories.
    no_ignore_parent: bool,
    #[structopt(
        long,
        raw(overrides_with = r#""no-ignore-parent""#),
        raw(hidden = "true")
    )]
    ignore_parent: bool,

    #[structopt(long, raw(overrides_with = r#""ignore-vcs""#))]
    /// Don't respect ignore files in vcs directories.
    no_ignore_vcs: bool,
    #[structopt(long, raw(overrides_with = r#""no-ignore-vcs""#), raw(hidden = "true"))]
    ignore_vcs: bool,
}

impl Options {
    pub fn infer(mut self) -> Self {
        if self.path.len() == 1 && self.path[0].is_file() {
            self.threads = 1;
        }

        self
    }

    pub fn ignore_hidden(&self) -> Option<bool> {
        match (self.hidden, self.no_hidden) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    pub fn ignore_dot(&self) -> Option<bool> {
        match (self.no_ignore_dot, self.ignore_dot) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
        .or_else(|| self.ignore_files())
    }

    pub fn ignore_global(&self) -> Option<bool> {
        match (self.no_ignore_global, self.ignore_global) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
        .or_else(|| self.ignore_vcs())
        .or_else(|| self.ignore_files())
    }

    pub fn ignore_parent(&self) -> Option<bool> {
        match (self.no_ignore_parent, self.ignore_parent) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
        .or_else(|| self.ignore_files())
    }

    pub fn ignore_vcs(&self) -> Option<bool> {
        match (self.no_ignore_vcs, self.ignore_vcs) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
        .or_else(|| self.ignore_files())
    }

    fn ignore_files(&self) -> Option<bool> {
        match (self.no_ignore, self.ignore) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }
}

fn run() -> Result<(), failure::Error> {
    let options = Options::from_args().infer();

    let dictionary = typos::Dictionary::new();

    let first_path = &options
        .path
        .get(0)
        .expect("arg parsing enforces at least one");
    let mut walk = ignore::WalkBuilder::new(first_path);
    for path in &options.path[1..] {
        walk.add(path);
    }
    walk.threads(options.threads)
        .hidden(options.ignore_hidden().unwrap_or(true))
        .ignore(options.ignore_dot().unwrap_or(true))
        .git_global(options.ignore_global().unwrap_or(true))
        .git_ignore(options.ignore_vcs().unwrap_or(true))
        .git_exclude(options.ignore_vcs().unwrap_or(true))
        .parents(options.ignore_parent().unwrap_or(true));
    // TODO Add build_parallel for options.threads != 1
    for entry in walk.build() {
        let entry = entry?;
        if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
            typos::process_file(entry.path(), &dictionary, options.format.report())?;
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
