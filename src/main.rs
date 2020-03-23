// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::io::Write;
use std::sync::atomic;

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

const PRINT_SILENT: typos::report::PrintSilent = typos::report::PrintSilent;
const PRINT_BRIEF: typos::report::PrintBrief = typos::report::PrintBrief;
const PRINT_LONG: typos::report::PrintLong = typos::report::PrintLong;
const PRINT_JSON: typos::report::PrintJson = typos::report::PrintJson;

impl Format {
    fn reporter(self) -> &'static dyn typos::report::Report {
        match self {
            Format::Silent => &PRINT_SILENT,
            Format::Brief => &PRINT_BRIEF,
            Format::Long => &PRINT_LONG,
            Format::Json => &PRINT_JSON,
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

    #[structopt(short = "j", long = "threads", default_value = "0")]
    /// The approximate number of threads to use.
    threads: usize,

    #[structopt(flatten)]
    config: ConfigArgs,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Args {
    pub fn infer(mut self) -> Self {
        if self.path.len() == 1 && self.path[0].is_file() {
            self.threads = 1;
        }

        self
    }
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

trait Checks: Send + Sync {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error>;

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error>;
}

impl<'p> Checks for typos::checks::ParseIdentifiers {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
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
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
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
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, dictionary, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, dictionary, report)
    }
}

fn init_logging(level: Option<log::Level>) {
    if let Some(level) = level {
        let mut builder = env_logger::Builder::new();

        builder.filter(None, level.to_level_filter());

        if level == log::LevelFilter::Trace {
            builder.format_timestamp_secs();
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

fn check_path(
    walk: ignore::Walk,
    format: Format,
    checks: &dyn Checks,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
) -> (bool, bool) {
    let mut typos_found = false;
    let mut errors_found = false;

    for entry in walk {
        match check_entry(entry, format, checks, parser, dictionary) {
            Ok(true) => typos_found = true,
            Err(err) => {
                let msg = typos::report::Error::new(err.to_string());
                format.reporter().report(msg.into());
                errors_found = true
            }
            _ => (),
        }
    }

    (typos_found, errors_found)
}

fn check_path_parallel(
    walk: ignore::WalkParallel,
    format: Format,
    checks: &dyn Checks,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
) -> (bool, bool) {
    let typos_found = atomic::AtomicBool::new(false);
    let errors_found = atomic::AtomicBool::new(false);

    walk.run(|| {
        Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
            match check_entry(entry, format, checks, parser, dictionary) {
                Ok(true) => typos_found.store(true, atomic::Ordering::Relaxed),
                Err(err) => {
                    let msg = typos::report::Error::new(err.to_string());
                    format.reporter().report(msg.into());
                    errors_found.store(true, atomic::Ordering::Relaxed);
                }
                _ => (),
            }
            ignore::WalkState::Continue
        })
    });

    (typos_found.into_inner(), errors_found.into_inner())
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
        if checks.check_filename(entry.path(), parser, dictionary, format.reporter())? {
            typos_found = true;
        }
        if checks.check_file(
            entry.path(),
            explicit,
            parser,
            dictionary,
            format.reporter(),
        )? {
            typos_found = true;
        }
    }

    Ok(typos_found)
}

fn run() -> Result<i32, anyhow::Error> {
    let args = Args::from_args().infer();

    init_logging(args.verbose.log_level());

    let config = if let Some(path) = args.custom_config.as_ref() {
        config::Config::from_file(path)?
    } else {
        config::Config::default()
    };

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
        walk.threads(args.threads)
            .hidden(config.files.ignore_hidden())
            .ignore(config.files.ignore_dot())
            .git_global(config.files.ignore_global())
            .git_ignore(config.files.ignore_vcs())
            .git_exclude(config.files.ignore_vcs())
            .parents(config.files.ignore_parent());
        let single_threaded = args.threads == 1;
        if args.files {
            if single_threaded {
                for entry in walk.build() {
                    match entry {
                        Ok(entry) => {
                            let msg = typos::report::File::new(entry.path());
                            args.format.reporter().report(msg.into());
                        }
                        Err(err) => {
                            let msg = typos::report::Error::new(err.to_string());
                            args.format.reporter().report(msg.into());
                            errors_found = true
                        }
                    }
                }
            } else {
                let format = args.format;
                let atomic_errors = atomic::AtomicBool::new(errors_found);
                walk.build_parallel().run(|| {
                    Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
                        match entry {
                            Ok(entry) => {
                                let msg = typos::report::File::new(entry.path());
                                format.reporter().report(msg.into());
                            }
                            Err(err) => {
                                let msg = typos::report::Error::new(err.to_string());
                                format.reporter().report(msg.into());
                                atomic_errors.store(true, atomic::Ordering::Relaxed);
                            }
                        }
                        ignore::WalkState::Continue
                    })
                });
                errors_found = atomic_errors.into_inner();
            }
        } else if args.identifiers {
            let checks = settings.build_identifier_parser();
            let (cur_typos, cur_errors) = if single_threaded {
                check_path(walk.build(), args.format, &checks, &parser, &dictionary)
            } else {
                check_path_parallel(
                    walk.build_parallel(),
                    args.format,
                    &checks,
                    &parser,
                    &dictionary,
                )
            };
            if cur_typos {
                typos_found = true;
            }
            if cur_errors {
                errors_found = true;
            }
        } else if args.words {
            let checks = settings.build_word_parser();
            let (cur_typos, cur_errors) = if single_threaded {
                check_path(walk.build(), args.format, &checks, &parser, &dictionary)
            } else {
                check_path_parallel(
                    walk.build_parallel(),
                    args.format,
                    &checks,
                    &parser,
                    &dictionary,
                )
            };
            if cur_typos {
                typos_found = true;
            }
            if cur_errors {
                errors_found = true;
            }
        } else {
            let checks = settings.build_checks();
            let (cur_typos, cur_errors) = if single_threaded {
                check_path(walk.build(), args.format, &checks, &parser, &dictionary)
            } else {
                check_path_parallel(
                    walk.build_parallel(),
                    args.format,
                    &checks,
                    &parser,
                    &dictionary,
                )
            };
            if cur_typos {
                typos_found = true;
            }
            if cur_errors {
                errors_found = true;
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
