use clap::Parser;

use typos_cli::config;

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ArgEnum)]
pub enum Format {
    Silent,
    Brief,
    Github,
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
            Format::Github => Box::new(crate::report::PrintGithub {
                stdout_palette,
                stderr_palette,
            }),
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
#[clap(rename_all = "kebab-case")]
#[clap(about, author, version)]
#[clap(
        setting = clap::AppSettings::DeriveDisplayOrder,
        setting = clap::AppSettings::DontCollapseArgsInUsage,
        color = concolor_clap::color_choice(),
    )]
#[clap(group = clap::ArgGroup::new("mode").multiple(false))]
pub(crate) struct Args {
    #[clap(parse(from_os_str), default_value = ".")]
    /// Paths to check with `-` for stdin
    pub(crate) path: Vec<std::path::PathBuf>,

    #[clap(short = 'c', long = "config", parse(from_os_str))]
    /// Custom config file
    pub(crate) custom_config: Option<std::path::PathBuf>,

    #[clap(long)]
    /// Ignore implicit configuration files.
    pub(crate) isolated: bool,

    #[clap(long, group = "mode")]
    /// Print a diff of what would change
    pub(crate) diff: bool,

    #[clap(long, short = 'w', group = "mode")]
    /// Write fixes out
    pub(crate) write_changes: bool,

    #[clap(long, group = "mode")]
    /// Debug: Print each file that would be spellchecked.
    pub(crate) files: bool,

    #[clap(long, group = "mode")]
    /// Debug: Print each identifier that would be spellchecked.
    pub(crate) identifiers: bool,

    #[clap(long, group = "mode")]
    /// Debug: Print each word that would be spellchecked.
    pub(crate) words: bool,

    #[clap(long, parse(from_os_str), group = "mode")]
    /// Write the current configuration to file with `-` for stdout
    pub(crate) dump_config: Option<std::path::PathBuf>,

    #[clap(long, group = "mode")]
    /// Show all supported file types.
    pub(crate) type_list: bool,

    #[clap(long, arg_enum, ignore_case = true, default_value("long"))]
    pub(crate) format: Format,

    #[clap(short = 'j', long = "threads", default_value = "0")]
    /// The approximate number of threads to use.
    pub(crate) threads: usize,

    #[clap(flatten)]
    pub(crate) config: ConfigArgs,

    #[clap(flatten)]
    pub(crate) color: concolor_clap::Color,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Clone, clap::Args)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct FileArgs {
    #[clap(long, overrides_with("no-binary"))]
    /// Search binary files.
    binary: bool,
    #[clap(long, overrides_with("binary"), hide = true)]
    no_binary: bool,

    #[clap(long, overrides_with("check-filenames"))]
    /// Skip verifying spelling in file names.
    no_check_filenames: bool,
    #[clap(long, overrides_with("no-check-filenames"), hide = true)]
    check_filenames: bool,

    #[clap(long, overrides_with("check-files"))]
    /// Skip verifying spelling in files.
    no_check_files: bool,
    #[clap(long, overrides_with("no-check-files"), hide = true)]
    check_files: bool,

    #[clap(long, overrides_with("no-unicode"), hide = true)]
    unicode: bool,
    #[clap(long, overrides_with("unicode"))]
    /// Only allow ASCII characters in identifiers
    no_unicode: bool,

    #[clap(long, possible_values(config::Locale::variants()))]
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

    #[clap(long, overrides_with("no-hidden"))]
    /// Search hidden files and directories.
    hidden: bool,
    #[clap(long, overrides_with("hidden"), hide = true)]
    no_hidden: bool,

    #[clap(long, overrides_with("ignore"))]
    /// Don't respect ignore files.
    no_ignore: bool,
    #[clap(long, overrides_with("no-ignore"), hide = true)]
    ignore: bool,

    #[clap(long, overrides_with("ignore-dot"))]
    /// Don't respect .ignore files.
    no_ignore_dot: bool,
    #[clap(long, overrides_with("no-ignore-dot"), hide = true)]
    ignore_dot: bool,

    #[clap(long, overrides_with("ignore-global"))]
    /// Don't respect global ignore files.
    no_ignore_global: bool,
    #[clap(long, overrides_with("no-ignore-global"), hide = true)]
    ignore_global: bool,

    #[clap(long, overrides_with("ignore-parent"))]
    /// Don't respect ignore files in parent directories.
    no_ignore_parent: bool,
    #[clap(long, overrides_with("no-ignore-parent"), hide = true)]
    ignore_parent: bool,

    #[clap(long, overrides_with("ignore-vcs"))]
    /// Don't respect ignore files in vcs directories.
    no_ignore_vcs: bool,
    #[clap(long, overrides_with("no-ignore-vcs"), hide = true)]
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
        use clap::IntoApp;
        Args::into_app().debug_assert()
    }
}
