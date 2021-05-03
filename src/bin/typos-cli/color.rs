use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct ColorArgs {
    /// "Specify when to use colored output. The automatic mode
    /// only enables colors if an interactive terminal is detected -
    /// colors are automatically disabled if the output goes to a pipe.
    ///
    /// Possible values: *auto*, never, always.
    #[structopt(
        long,
        value_name="when",
        possible_values(&ColorValue::variants()),
        case_insensitive(true),
        default_value("auto"),
        hide_possible_values(true),
        hide_default_value(true),
        help="When to use colors (*auto*, never, always).")]
    color: ColorValue,
}

impl ColorArgs {
    pub fn colored(&self) -> Option<bool> {
        self.color.colored()
    }
}

arg_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ColorValue {
        Always,
        Never,
        Auto,
    }
}

impl ColorValue {
    fn colored(self) -> Option<bool> {
        match self {
            ColorValue::Always => Some(true),
            ColorValue::Never => Some(false),
            ColorValue::Auto => None,
        }
    }
}

impl Default for ColorValue {
    fn default() -> Self {
        ColorValue::Auto
    }
}

pub fn colored_stdout() -> Option<bool> {
    if atty::is(atty::Stream::Stdout) {
        None
    } else {
        Some(false)
    }
}

pub fn colored_stderr() -> Option<bool> {
    if atty::is(atty::Stream::Stderr) {
        None
    } else {
        Some(false)
    }
}

pub fn colored_env() -> Option<bool> {
    match std::env::var_os("TERM") {
        None => noterm_colored(),
        Some(k) => {
            if k == "dumb" {
                Some(false)
            } else {
                None
            }
        }
    }
    .or_else(|| {
        // See https://no-color.org/
        std::env::var_os("NO_COLOR").map(|_| true)
    })
}

#[cfg(not(windows))]
fn noterm_colored() -> Option<bool> {
    // If TERM isn't set, then we are in a weird environment that
    // probably doesn't support colors.
    Some(false)
}

#[cfg(windows)]
fn noterm_colored() -> Option<bool> {
    // On Windows, if TERM isn't set, then we shouldn't automatically
    // assume that colors aren't allowed. This is unlike Unix environments
    // where TERM is more rigorously set.
    None
}
