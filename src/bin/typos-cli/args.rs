use structopt::StructOpt;

use typos_cli::config;

arg_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Format {
        Silent,
        Brief,
        Long,
        Json,
    }
}

pub const PRINT_SILENT: crate::report::PrintSilent = crate::report::PrintSilent;
pub const PRINT_BRIEF: crate::report::PrintBrief = crate::report::PrintBrief;
pub const PRINT_LONG: crate::report::PrintLong = crate::report::PrintLong;
pub const PRINT_JSON: crate::report::PrintJson = crate::report::PrintJson;

impl Format {
    pub(crate) fn reporter(self) -> &'static dyn typos_cli::report::Report {
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
#[structopt(group = structopt::clap::ArgGroup::with_name("mode").multiple(false))]
pub(crate) struct Args {
    #[structopt(parse(from_os_str), default_value = ".")]
    /// Paths to check with `-` for stdin
    pub(crate) path: Vec<std::path::PathBuf>,

    #[structopt(short = "c", long = "config")]
    /// Custom config file
    pub(crate) custom_config: Option<std::path::PathBuf>,

    #[structopt(long)]
    /// Ignore implicit configuration files.
    pub(crate) isolated: bool,

    #[structopt(long, group = "mode")]
    /// Print a diff of what would change
    pub(crate) diff: bool,

    #[structopt(long, short = "w", group = "mode")]
    /// Write fixes out
    pub(crate) write_changes: bool,

    #[structopt(long, group = "mode")]
    /// Debug: Print each file that would be spellchecked.
    pub(crate) files: bool,

    #[structopt(long, group = "mode")]
    /// Debug: Print each identifier that would be spellchecked.
    pub(crate) identifiers: bool,

    #[structopt(long, group = "mode")]
    /// Debug: Print each word that would be spellchecked.
    pub(crate) words: bool,

    #[structopt(long, group = "mode")]
    /// Write the current configuration to file with `-` for stdout
    pub(crate) dump_config: Option<std::path::PathBuf>,

    #[structopt(long, group = "mode")]
    /// Show all supported file types.
    pub(crate) type_list: bool,

    #[structopt(
        long,
        possible_values(&Format::variants()),
        case_insensitive(true),
        default_value("long")
    )]
    pub(crate) format: Format,

    #[structopt(short = "j", long = "threads", default_value = "0")]
    /// The approximate number of threads to use.
    pub(crate) threads: usize,

    #[structopt(flatten)]
    pub(crate) config: ConfigArgs,

    #[structopt(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct FileArgs {
    #[structopt(long, overrides_with("no-binary"))]
    /// Search binary files.
    binary: bool,
    #[structopt(long, overrides_with("binary"), hidden(true))]
    no_binary: bool,

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

    #[structopt(long, overrides_with("no-unicode"), hidden(true))]
    unicode: bool,
    #[structopt(long, overrides_with("unicode"))]
    /// Only allow ASCII characters in identifiers
    no_unicode: bool,

    #[structopt(
        long,
        possible_values(&config::Locale::variants()),
    )]
    pub(crate) locale: Option<config::Locale>,
}

impl FileArgs {
    pub fn to_config(&self) -> config::EngineConfig {
        config::EngineConfig {
            binary: self.binary(),
            check_filename: self.check_filename(),
            check_file: self.check_file(),
            tokenizer: Some(config::TokenizerConfig {
                unicode: self.unicode(),
                ..Default::default()
            }),
            dict: Some(config::DictConfig {
                locale: self.locale,
                ..Default::default()
            }),
        }
    }

    fn binary(&self) -> Option<bool> {
        resolve_bool_arg(self.binary, self.no_binary)
    }

    fn check_filename(&self) -> Option<bool> {
        resolve_bool_arg(self.check_filenames, self.no_check_filenames)
    }

    fn unicode(&self) -> Option<bool> {
        resolve_bool_arg(self.unicode, self.no_unicode)
    }

    fn check_file(&self) -> Option<bool> {
        resolve_bool_arg(self.check_files, self.no_check_files)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct ConfigArgs {
    #[structopt(flatten)]
    walk: WalkArgs,
    #[structopt(flatten)]
    overrides: FileArgs,
}

impl ConfigArgs {
    pub fn to_config(&self) -> config::Config {
        config::Config {
            files: self.walk.to_config(),
            overrides: self.overrides.to_config(),
            ..Default::default()
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct WalkArgs {
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

impl WalkArgs {
    pub fn to_config(&self) -> config::Walk {
        config::Walk {
            ignore_hidden: self.ignore_hidden(),
            ignore_files: self.ignore_files(),
            ignore_dot: self.ignore_dot(),
            ignore_vcs: self.ignore_vcs(),
            ignore_global: self.ignore_global(),
            ignore_parent: self.ignore_parent(),
        }
    }

    fn ignore_hidden(&self) -> Option<bool> {
        resolve_bool_arg(self.no_hidden, self.hidden)
    }

    fn ignore_files(&self) -> Option<bool> {
        resolve_bool_arg(self.ignore, self.no_ignore)
    }

    fn ignore_dot(&self) -> Option<bool> {
        resolve_bool_arg(self.ignore_dot, self.no_ignore_dot)
    }

    fn ignore_vcs(&self) -> Option<bool> {
        resolve_bool_arg(self.ignore_vcs, self.no_ignore_vcs)
    }

    fn ignore_global(&self) -> Option<bool> {
        resolve_bool_arg(self.ignore_global, self.no_ignore_global)
    }

    fn ignore_parent(&self) -> Option<bool> {
        resolve_bool_arg(self.ignore_parent, self.no_ignore_parent)
    }
}

fn resolve_bool_arg(yes: bool, no: bool) -> Option<bool> {
    match (yes, no) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        (false, false) => None,
        (_, _) => unreachable!("StructOpt should make this impossible"),
    }
}
