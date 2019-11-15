// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::io::Write;

use structopt::StructOpt;

mod config;
mod dict;

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
#[structopt(
        setting = structopt::clap::AppSettings::UnifiedHelpMessage,
        setting = structopt::clap::AppSettings::DeriveDisplayOrder,
        setting = structopt::clap::AppSettings::DontCollapseArgsInUsage
    )]
struct Args {
    #[structopt(parse(from_os_str), default_value = ".")]
    /// Paths to check
    path: Vec<std::path::PathBuf>,

    #[structopt(short = "c", long = "config")]
    /// Custom config file
    custom_config: Option<std::path::PathBuf>,

    #[structopt(long)]
    /// Ignore implicit configuration files.
    isolated: bool,

    #[structopt(long)]
    /// Print each file that would be spellchecked.
    files: bool,

    #[structopt(long)]
    /// Print each identifier that would be spellchecked.
    identifiers: bool,

    #[structopt(long)]
    /// Print each word that would be spellchecked.
    words: bool,

    #[structopt(flatten)]
    overrides: FileArgs,

    #[structopt(
        long,
        possible_values(&Format::variants()),
        case_insensitive(true),
        default_value("long")
    )]
    pub format: Format,

    #[structopt(flatten)]
    config: ConfigArgs,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct FileArgs {
    #[structopt(long, overrides_with("check-filenames"))]
    /// Skip verifying spelling in file names.
    no_check_filenames: bool,
    #[structopt(long, overrides_with("no-check-filenames"), hidden(true))]
    check_filenames: bool,

    #[structopt(long, overrides_with("check-files"))]
    /// Skip verifying spelling in filess.
    no_check_files: bool,
    #[structopt(long, overrides_with("no-check-files"), hidden(true))]
    check_files: bool,

    #[structopt(long, overrides_with("hex"))]
    /// Don't try to detect that an identifier looks like hex
    no_hex: bool,
    #[structopt(long, overrides_with("no-hex"), hidden(true))]
    hex: bool,
}

impl config::FileSource for FileArgs {
    fn check_filename(&self) -> Option<bool> {
        match (self.check_filenames, self.no_check_filenames) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn check_file(&self) -> Option<bool> {
        match (self.check_files, self.no_check_files) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_hex(&self) -> Option<bool> {
        match (self.hex, self.no_hex) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct ConfigArgs {
    #[structopt(flatten)]
    walk: WalkArgs,
}

impl config::ConfigSource for ConfigArgs {
    fn walk(&self) -> Option<&dyn config::WalkSource> {
        Some(&self.walk)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct WalkArgs {
    #[structopt(long, overrides_with("no-binary"))]
    /// Search binary files.
    binary: bool,
    #[structopt(long, overrides_with("binary"), hidden(true))]
    no_binary: bool,

    #[structopt(long, overrides_with("no-hidden"))]
    /// Search hidden files and directories.
    hidden: bool,
    #[structopt(long, overrides_with("hidden"), hidden(true))]
    no_hidden: bool,

    #[structopt(long, overrides_with("ignore"))]
    /// Don't respect ignore files.
    no_ignore: bool,
    #[structopt(long, overrides_with("no-ignore"), hidden(true))]
    ignore: bool,

    #[structopt(long, overrides_with("ignore-dot"))]
    /// Don't respect .ignore files.
    no_ignore_dot: bool,
    #[structopt(long, overrides_with("no-ignore-dot"), hidden(true))]
    ignore_dot: bool,

    #[structopt(long, overrides_with("ignore-global"))]
    /// Don't respect global ignore files.
    no_ignore_global: bool,
    #[structopt(long, overrides_with("no-ignore-global"), hidden(true))]
    ignore_global: bool,

    #[structopt(long, overrides_with("ignore-parent"))]
    /// Don't respect ignore files in parent directories.
    no_ignore_parent: bool,
    #[structopt(long, overrides_with("no-ignore-parent"), hidden(true))]
    ignore_parent: bool,

    #[structopt(long, overrides_with("ignore-vcs"))]
    /// Don't respect ignore files in vcs directories.
    no_ignore_vcs: bool,
    #[structopt(long, overrides_with("no-ignore-vcs"), hidden(true))]
    ignore_vcs: bool,
}

impl config::WalkSource for WalkArgs {
    fn binary(&self) -> Option<bool> {
        match (self.binary, self.no_binary) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

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

trait Checks {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error>;

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error>;
}

impl<'p> Checks for typos::checks::ParseIdentifiers {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, report)
    }
}

impl<'p> Checks for typos::checks::ParseWords {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, report)
    }
}

impl<'d, 'p> Checks for typos::checks::Checks {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, dictionary, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, dictionary, report)
    }
}

fn init_logging(level: Option<log::Level>) {
    if let Some(level) = level {
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

        builder.init();
    }
}

fn check_entry(
    entry: Result<ignore::DirEntry, ignore::Error>,
    format: Format,
    checks: &dyn Checks,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
) -> Result<bool, anyhow::Error> {
    let mut typos_found = false;

    let entry = entry?;
    if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
        let explicit = entry.depth() == 0;
        if checks.check_filename(entry.path(), parser, dictionary, format.report())? {
            typos_found = true;
        }
        if checks.check_file(entry.path(), explicit, parser, dictionary, format.report())? {
            typos_found = true;
        }
    }

    Ok(typos_found)
}

fn run() -> Result<i32, anyhow::Error> {
    let args = Args::from_args();

    init_logging(args.verbose.log_level());

    let mut config = config::Config::default();
    if let Some(path) = args.custom_config.as_ref() {
        let custom = config::Config::from_file(path)?;
        config.update(&custom);
    }
    let config = config;

    let mut typos_found = false;
    let mut errors_found = false;
    for path in args.path.iter() {
        let path = path.canonicalize()?;
        let cwd = if path.is_file() {
            path.parent().unwrap()
        } else {
            path.as_path()
        };

        let mut config = config.clone();
        if !args.isolated {
            let derived = config::Config::derive(cwd)?;
            config.update(&derived);
        }
        config.update(&args.config);
        config.default.update(&args.overrides);
        let config = config;

        let parser = typos::tokens::ParserBuilder::new()
            .ignore_hex(config.default.ignore_hex())
            .leading_digits(config.default.identifier_leading_digits())
            .leading_chars(config.default.identifier_leading_chars().to_owned())
            .include_digits(config.default.identifier_include_digits())
            .include_chars(config.default.identifier_include_chars().to_owned())
            .build();

        let dictionary = crate::dict::BuiltIn::new();

        let mut settings = typos::checks::TyposSettings::new();
        settings
            .check_filenames(config.default.check_filename())
            .check_files(config.default.check_file())
            .binary(config.files.binary());

        let mut walk = ignore::WalkBuilder::new(path);
        walk.hidden(config.files.ignore_hidden())
            .ignore(config.files.ignore_dot())
            .git_global(config.files.ignore_global())
            .git_ignore(config.files.ignore_vcs())
            .git_exclude(config.files.ignore_vcs())
            .parents(config.files.ignore_parent());
        if args.files {
            for entry in walk.build() {
                match entry {
                    Ok(entry) => {
                        let msg = typos::report::File::new(entry.path());
                        args.format.report()(msg.into());
                    }
                    Err(err) => {
                        let msg = typos::report::Error::new(err.to_string());
                        args.format.report()(msg.into());
                        errors_found = true
                    }
                }
            }
        } else if args.identifiers {
            let checks = settings.build_identifier_parser();
            for entry in walk.build() {
                match check_entry(entry, args.format, &checks, &parser, &dictionary) {
                    Ok(true) => typos_found = true,
                    Err(err) => {
                        let msg = typos::report::Error::new(err.to_string());
                        args.format.report()(msg.into());
                        errors_found = true
                    }
                    _ => (),
                }
            }
        } else if args.words {
            let checks = settings.build_word_parser();
            for entry in walk.build() {
                match check_entry(entry, args.format, &checks, &parser, &dictionary) {
                    Ok(true) => typos_found = true,
                    Err(err) => {
                        let msg = typos::report::Error::new(err.to_string());
                        args.format.report()(msg.into());
                        errors_found = true
                    }
                    _ => (),
                }
            }
        } else {
            let checks = settings.build_checks();
            for entry in walk.build() {
                match check_entry(entry, args.format, &checks, &parser, &dictionary) {
                    Ok(true) => typos_found = true,
                    Err(err) => {
                        let msg = typos::report::Error::new(err.to_string());
                        args.format.report()(msg.into());
                        errors_found = true
                    }
                    _ => (),
                }
            }
        }
    }

    if errors_found {
        Ok(2)
    } else if typos_found {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
