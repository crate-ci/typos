use clap::builder::TypedValueParser;
use clap::Parser;

use typos_cli::config;

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum Format {
    Silent,
    Brief,
    #[default]
    Long,
    Json,
}

impl Format {
    pub(crate) fn reporter(
        self,
        stdout_palette: crate::report::Palette,
        stderr_palette: crate::report::Palette,
    ) -> Box<dyn typos_cli::report::Report> {
        match self {
            Format::Silent => Box::new(crate::report::PrintSilent),
            Format::Brief => Box::new(crate::report::PrintBrief {
                stdout_palette,
                stderr_palette,
            }),
            Format::Long => Box::new(crate::report::PrintLong {
                stdout_palette,
                stderr_palette,
            }),
            Format::Json => Box::new(crate::report::PrintJson),
        }
    }
}

#[derive(Debug, Parser)]
#[command(rename_all = "kebab-case")]
#[command(about, author, version)]
#[command(group = clap::ArgGroup::new("mode").multiple(false))]
pub(crate) struct Args {
    /// Paths to check with `-` for stdin
    #[arg(default_value = ".")]
    pub(crate) path: Vec<std::path::PathBuf>,

    /// The approximate number of threads to use.
    #[arg(short = 'j', long = "threads", default_value = "0")]
    pub(crate) threads: usize,

    /// Respect excluded files even for paths passed explicitly.
    #[arg(long, help_heading = None)]
    pub(crate) force_exclude: bool,

    /// Custom config file
    #[arg(short = 'c', long = "config", help_heading = "Config")]
    pub(crate) custom_config: Option<std::path::PathBuf>,

    /// Ignore implicit configuration files.
    #[arg(long, help_heading = "Config")]
    pub(crate) isolated: bool,

    #[command(flatten, next_help_heading = "Config")]
    pub(crate) config: ConfigArgs,

    /// Print a diff of what would change
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) diff: bool,

    /// Write fixes out
    #[arg(long, short = 'w', group = "mode", help_heading = "Mode")]
    pub(crate) write_changes: bool,

    /// Debug: Print each file that would be spellchecked.
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) files: bool,

    /// Debug: Print each file's type
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) file_types: bool,

    /// Debug: Print each identifier that would be spellchecked.
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) identifiers: bool,

    /// Debug: Print each word that would be spellchecked.
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) words: bool,

    /// Write the current configuration to file with `-` for stdout
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) dump_config: Option<std::path::PathBuf>,

    /// Show all supported file types.
    #[arg(long, group = "mode", help_heading = "Mode")]
    pub(crate) type_list: bool,

    /// Render style for messages
    #[arg(
        long,
        value_enum,
        ignore_case = true,
        default_value("long"),
        help_heading = "Output"
    )]
    pub(crate) format: Format,

    #[command(flatten, next_help_heading = "Output")]
    pub(crate) color: colorchoice_clap::Color,

    #[command(flatten, next_help_heading = "Output")]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Clone, clap::Args)]
#[command(rename_all = "kebab-case")]
pub(crate) struct FileArgs {
    /// Search binary files.
    #[arg(long, overrides_with("no_binary"))]
    binary: bool,
    #[arg(long, overrides_with("binary"), hide = true)]
    no_binary: bool,

    /// Skip verifying spelling in file names.
    #[arg(long, overrides_with("check_filenames"))]
    no_check_filenames: bool,
    #[arg(long, overrides_with("no_check_filenames"), hide = true)]
    check_filenames: bool,

    /// Skip verifying spelling in files.
    #[arg(long, overrides_with("check_files"))]
    no_check_files: bool,
    #[arg(long, overrides_with("no_check_files"), hide = true)]
    check_files: bool,

    #[arg(long, overrides_with("no_unicode"), hide = true)]
    unicode: bool,
    /// Only allow ASCII characters in identifiers
    #[arg(long, overrides_with("unicode"))]
    no_unicode: bool,

    /// Language locale to suggest corrections for
    #[arg(long)]
    #[arg(
        value_parser = clap::builder::PossibleValuesParser::new(config::Locale::variants())
            .map(|l| l.parse::<config::Locale>().unwrap())
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
            extend_ignore_re: Default::default(),
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

#[derive(Debug, clap::Args)]
#[command(rename_all = "kebab-case")]
pub(crate) struct ConfigArgs {
    #[command(flatten)]
    walk: WalkArgs,
    #[command(flatten)]
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

#[derive(Debug, clap::Args)]
#[command(rename_all = "kebab-case")]
pub(crate) struct WalkArgs {
    /// Ignore files & directories matching the glob.
    #[arg(long, value_name = "GLOB")]
    exclude: Vec<String>,

    /// Search hidden files and directories.
    #[arg(long, overrides_with("no_hidden"))]
    hidden: bool,
    #[arg(long, overrides_with("hidden"), hide = true)]
    no_hidden: bool,

    /// Don't respect ignore files.
    #[arg(long, overrides_with("ignore"))]
    no_ignore: bool,
    #[arg(long, overrides_with("no_ignore"), hide = true)]
    ignore: bool,

    /// Don't respect .ignore files.
    #[arg(long, overrides_with("ignore_dot"))]
    no_ignore_dot: bool,
    #[arg(long, overrides_with("no_ignore_dot"), hide = true)]
    ignore_dot: bool,

    /// Don't respect global ignore files.
    #[arg(long, overrides_with("ignore_global"))]
    no_ignore_global: bool,
    #[arg(long, overrides_with("no_ignore_global"), hide = true)]
    ignore_global: bool,

    /// Don't respect ignore files in parent directories.
    #[arg(long, overrides_with("ignore_parent"))]
    no_ignore_parent: bool,
    #[arg(long, overrides_with("no_ignore_parent"), hide = true)]
    ignore_parent: bool,

    /// Don't respect ignore files in vcs directories.
    #[arg(long, overrides_with("ignore_vcs"))]
    no_ignore_vcs: bool,
    #[arg(long, overrides_with("no_ignore_vcs"), hide = true)]
    ignore_vcs: bool,
}

impl WalkArgs {
    pub fn to_config(&self) -> config::Walk {
        config::Walk {
            extend_exclude: self.exclude.clone(),
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
        (_, _) => unreachable!("clap should make this impossible"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
