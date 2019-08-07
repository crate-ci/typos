// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::io::Write;

use structopt::StructOpt;

mod config;

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

    #[structopt(short = "c", long = "config")]
    /// Custom config file
    custom_config: Option<std::path::PathBuf>,

    #[structopt(long = "isolated")]
    /// Ignore implicit configuration files.
    isolated: bool,

    #[structopt(long, raw(overrides_with = r#""check-filenames""#))]
    /// Skip verifying spelling in file names.
    no_check_filenames: bool,
    #[structopt(
        long,
        raw(overrides_with = r#""no-check-filenames""#),
        raw(hidden = "true")
    )]
    check_filenames: bool,

    #[structopt(long, raw(overrides_with = r#""check-files""#))]
    /// Skip verifying spelling in filess.
    no_check_files: bool,
    #[structopt(
        long,
        raw(overrides_with = r#""no-check-files""#),
        raw(hidden = "true")
    )]
    check_files: bool,

    #[structopt(long, raw(overrides_with = r#""hex""#))]
    /// Don't try to detect that an identifier looks like hex
    no_hex: bool,
    #[structopt(long, raw(overrides_with = r#""no-hex""#), raw(hidden = "true"))]
    hex: bool,

    #[structopt(
        long = "format",
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
        default_value = "long"
    )]
    pub format: Format,

    #[structopt(long, raw(overrides_with = r#""no-binary""#))]
    /// Search binary files.
    binary: bool,
    #[structopt(long, raw(overrides_with = r#""binary""#), raw(hidden = "true"))]
    no_binary: bool,

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

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Options {
    pub fn infer(self) -> Self {
        self
    }

    pub fn check_files(&self) -> Option<bool> {
        match (self.check_files, self.no_check_files) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    pub fn check_filenames(&self) -> Option<bool> {
        match (self.check_filenames, self.no_check_filenames) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    pub fn ignore_hex(&self) -> Option<bool> {
        match (self.no_hex, self.hex) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    pub fn binary(&self) -> Option<bool> {
        match (self.binary, self.no_binary) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }
}

impl config::ConfigSource for Options {
    fn ignore_hidden(&self) -> Option<bool> {
        match (self.hidden, self.no_hidden) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_files(&self) -> Option<bool> {
        match (self.no_ignore, self.ignore) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_dot(&self) -> Option<bool> {
        match (self.no_ignore_dot, self.ignore_dot) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_vcs(&self) -> Option<bool> {
        match (self.no_ignore_vcs, self.ignore_vcs) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_global(&self) -> Option<bool> {
        match (self.no_ignore_global, self.ignore_global) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_parent(&self) -> Option<bool> {
        match (self.no_ignore_parent, self.ignore_parent) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }
}

pub fn get_logging(level: log::Level) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level.to_level_filter());

    if level == log::LevelFilter::Trace {
        builder.default_format_timestamp(false);
    } else {
        builder.format(|f, record| {
            writeln!(
                f,
                "[{}] {}",
                record.level().to_string().to_lowercase(),
                record.args()
            )
        });
    }

    builder
}

fn run() -> Result<i32, failure::Error> {
    let options = Options::from_args().infer();

    let mut builder = get_logging(options.verbose.log_level());
    builder.init();

    let dictionary = typos::BuiltIn::new();

    let parser = typos::tokens::ParserBuilder::new()
        .ignore_hex(options.ignore_hex().unwrap_or(true))
        .build();

    let checks = typos::checks::CheckSettings::new()
        .check_filenames(options.check_filenames().unwrap_or(true))
        .check_files(options.check_files().unwrap_or(true))
        .binary(options.binary().unwrap_or(false))
        .build(&dictionary, &parser);

    let mut config = config::Config::default();
    if let Some(path) = options.custom_config.as_ref() {
        let custom = config::Config::from_file(path)?;
        config.update(&custom);
    }
    let config = config;

    let mut typos_found = false;
    for path in options.path.iter() {
        let path = path.canonicalize()?;
        let cwd = if path.is_file() {
            path.parent().unwrap()
        } else {
            path.as_path()
        };

        let mut config = config.clone();
        if !options.isolated {
            let derived = config::Config::derive(cwd)?;
            config.update(&derived);
        }
        config.update(&options);
        let config = config;

        let mut walk = ignore::WalkBuilder::new(path);
        walk.hidden(config.ignore_hidden())
            .ignore(config.ignore_dot())
            .git_global(config.ignore_global())
            .git_ignore(config.ignore_vcs())
            .git_exclude(config.ignore_vcs())
            .parents(config.ignore_parent());
        for entry in walk.build() {
            let entry = entry?;
            if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
                let explicit = entry.depth() == 0;
                if checks.check_filename(entry.path(), options.format.report())? {
                    typos_found = true;
                }
                if checks.check_file(entry.path(), explicit, options.format.report())? {
                    typos_found = true;
                }
            }
        }
    }

    if typos_found {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
