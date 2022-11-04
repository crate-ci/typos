use clap::builder::TypedValueParser;
use clap::Parser;

use typos_cli::config;

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Silent,
    Brief,
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

impl Default for Format {
    fn default() -> Self {
        Format::Long
    }
}

#[derive(Debug, Parser)]
#[command(rename_all = "kebab-case")]
#[command(about, author, version)]
#[command(
        color = concolor_clap::color_choice(),
    )]
#[command(group = clap::ArgGroup::new("mode").multiple(false))]
pub(crate) struct Args {
    #[arg(default_value = ".")]
    /// Paths to check with `-` for stdin
    pub(crate) path: Vec<std::path::PathBuf>,

    #[arg(short = 'c', long = "config")]
    /// Custom config file
    pub(crate) custom_config: Option<std::path::PathBuf>,

    #[arg(long)]
    /// Ignore implicit configuration files.
    pub(crate) isolated: bool,

    #[arg(long, group = "mode")]
    /// Print a diff of what would change
    pub(crate) diff: bool,

    #[arg(long, short = 'w', group = "mode")]
    /// Write fixes out
    pub(crate) write_changes: bool,

    #[arg(long, group = "mode")]
    /// Debug: Print each file that would be spellchecked.
    pub(crate) files: bool,

    #[arg(long, group = "mode")]
    /// Debug: Print each identifier that would be spellchecked.
    pub(crate) identifiers: bool,

    #[arg(long, group = "mode")]
    /// Debug: Print each word that would be spellchecked.
    pub(crate) words: bool,

    #[arg(long, group = "mode")]
    /// Write the current configuration to file with `-` for stdout
    pub(crate) dump_config: Option<std::path::PathBuf>,

    #[arg(long, group = "mode")]
    /// Show all supported file types.
    pub(crate) type_list: bool,

    #[arg(long, value_enum, ignore_case = true, default_value("long"))]
    pub(crate) format: Format,

    #[arg(short = 'j', long = "threads", default_value = "0")]
    /// The approximate number of threads to use.
    pub(crate) threads: usize,

    #[command(flatten)]
    pub(crate) config: ConfigArgs,

    #[command(flatten)]
    pub(crate) color: concolor_clap::Color,

    #[command(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Clone, clap::Args)]
#[command(rename_all = "kebab-case")]
pub(crate) struct FileArgs {
    #[arg(long, overrides_with("no_binary"))]
    /// Search binary files.
    binary: bool,
    #[arg(long, overrides_with("binary"), hide = true)]
    no_binary: bool,

    #[arg(long, overrides_with("check_filenames"))]
    /// Skip verifying spelling in file names.
    no_check_filenames: bool,
    #[arg(long, overrides_with("no_check_filenames"), hide = true)]
    check_filenames: bool,

    #[arg(long, overrides_with("check_files"))]
    /// Skip verifying spelling in files.
    no_check_files: bool,
    #[arg(long, overrides_with("no_check_files"), hide = true)]
    check_files: bool,

    #[arg(long, overrides_with("no_unicode"), hide = true)]
    unicode: bool,
    #[arg(long, overrides_with("unicode"))]
    /// Only allow ASCII characters in identifiers
    no_unicode: bool,

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
#[clap(rename_all = "kebab-case")]
pub(crate) struct ConfigArgs {
    #[clap(flatten)]
    walk: WalkArgs,
    #[clap(flatten)]
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
#[clap(rename_all = "kebab-case")]
pub(crate) struct WalkArgs {
    #[clap(long, name = "GLOB")]
    /// Ignore files & directories matching the glob.
    exclude: Vec<String>,

    #[clap(long, overrides_with("no_hidden"))]
    /// Search hidden files and directories.
    hidden: bool,
    #[clap(long, overrides_with("hidden"), hide = true)]
    no_hidden: bool,

    #[clap(long, overrides_with("ignore"))]
    /// Don't respect ignore files.
    no_ignore: bool,
    #[clap(long, overrides_with("no_ignore"), hide = true)]
    ignore: bool,

    #[clap(long, overrides_with("ignore_dot"))]
    /// Don't respect .ignore files.
    no_ignore_dot: bool,
    #[clap(long, overrides_with("no_ignore_dot"), hide = true)]
    ignore_dot: bool,

    #[clap(long, overrides_with("ignore_global"))]
    /// Don't respect global ignore files.
    no_ignore_global: bool,
    #[clap(long, overrides_with("no_ignore_global"), hide = true)]
    ignore_global: bool,

    #[clap(long, overrides_with("ignore_parent"))]
    /// Don't respect ignore files in parent directories.
    no_ignore_parent: bool,
    #[clap(long, overrides_with("no_ignore_parent"), hide = true)]
    ignore_parent: bool,

    #[clap(long, overrides_with("ignore_vcs"))]
    /// Don't respect ignore files in vcs directories.
    no_ignore_vcs: bool,
    #[clap(long, overrides_with("no_ignore_vcs"), hide = true)]
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
