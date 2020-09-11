use std::io::Read;

pub trait ConfigSource {
    fn walk(&self) -> Option<&dyn WalkSource> {
        None
    }

    fn default(&self) -> Option<&dyn FileSource> {
        None
    }
}

pub trait WalkSource {
    /// Search binary files.
    fn binary(&self) -> Option<bool> {
        None
    }

    /// The root for `ignore_patterns`
    fn ignore_root(&self) -> Option<&std::path::Path> {
        None
    }

    /// Ignore the specified patterns (gitignore syntax)
    fn ignore_patterns(&self) -> Option<&[String]> {
        None
    }

    /// Skip hidden files and directories.
    fn ignore_hidden(&self) -> Option<bool> {
        None
    }

    /// Respect ignore files.
    fn ignore_files(&self) -> Option<bool> {
        None
    }

    /// Respect .ignore files.
    fn ignore_dot(&self) -> Option<bool> {
        None
    }

    /// Respect ignore files in vcs directories.
    fn ignore_vcs(&self) -> Option<bool> {
        None
    }

    /// Respect global ignore files.
    fn ignore_global(&self) -> Option<bool> {
        None
    }

    /// Respect ignore files in parent directories.
    fn ignore_parent(&self) -> Option<bool> {
        None
    }
}

pub trait FileSource {
    /// Verifying spelling in file names.
    fn check_filename(&self) -> Option<bool> {
        None
    }

    /// Verifying spelling in files.
    fn check_file(&self) -> Option<bool> {
        None
    }

    /// Do not check identifiers that appear to be hexadecimal values.
    fn ignore_hex(&self) -> Option<bool> {
        None
    }

    /// Allow identifiers to start with digits, in addition to letters.
    fn identifier_leading_digits(&self) -> Option<bool> {
        None
    }

    /// Allow identifiers to start with one of these characters.
    fn identifier_leading_chars(&self) -> Option<&str> {
        None
    }

    /// Allow identifiers to include digits, in addition to letters.
    fn identifier_include_digits(&self) -> Option<bool> {
        None
    }

    /// Allow identifiers to include these characters.
    fn identifier_include_chars(&self) -> Option<&str> {
        None
    }

    fn locale(&self) -> Option<Locale> {
        None
    }

    fn extend_valid_identifiers(&self) -> &[String] {
        &[]
    }

    fn extend_valid_words(&self) -> &[String] {
        &[]
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub files: Walk,
    pub default: FileConfig,
}

impl Config {
    pub fn from_file(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let mut c = Self::from_toml(&s)?;
        c.files.ignore_root = path.parent().map(|p| p.to_owned());
        Ok(c)
    }

    pub fn from_toml(data: &str) -> Result<Self, anyhow::Error> {
        let content = toml::from_str(data)?;
        Ok(content)
    }

    pub fn derive(cwd: &std::path::Path) -> Result<Self, anyhow::Error> {
        if let Some(path) = find_project_file(cwd.to_owned(), "typos.toml") {
            Self::from_file(&path)
        } else {
            Ok(Default::default())
        }
    }

    pub fn update(&mut self, source: &dyn ConfigSource) {
        if let Some(walk) = source.walk() {
            self.files.update(walk);
        }
        if let Some(default) = source.default() {
            self.default.update(default);
        }
    }
}

impl ConfigSource for Config {
    fn walk(&self) -> Option<&dyn WalkSource> {
        Some(&self.files)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Walk {
    pub binary: Option<bool>,
    #[serde(skip)]
    pub ignore_root: Option<std::path::PathBuf>,
    pub ignore_patterns: Option<Vec<String>>,
    pub ignore_hidden: Option<bool>,
    pub ignore_files: Option<bool>,
    pub ignore_dot: Option<bool>,
    pub ignore_vcs: Option<bool>,
    pub ignore_global: Option<bool>,
    pub ignore_parent: Option<bool>,
}

impl Walk {
    pub fn update(&mut self, source: &dyn WalkSource) {
        if let Some(source) = source.binary() {
            self.binary = Some(source);
        }
        if let (Some(root), Some(source)) = (source.ignore_root(), source.ignore_patterns()) {
            self.ignore_root = Some(root.to_owned());
            self.ignore_patterns = Some(source.to_owned());
        }
        if let Some(source) = source.ignore_hidden() {
            self.ignore_hidden = Some(source);
        }
        if let Some(source) = source.ignore_files() {
            self.ignore_files = Some(source);
            self.ignore_dot = None;
            self.ignore_vcs = None;
            self.ignore_global = None;
            self.ignore_parent = None;
        }
        if let Some(source) = source.ignore_dot() {
            self.ignore_dot = Some(source);
        }
        if let Some(source) = source.ignore_vcs() {
            self.ignore_vcs = Some(source);
            self.ignore_global = None;
        }
        if let Some(source) = source.ignore_global() {
            self.ignore_global = Some(source);
        }
        if let Some(source) = source.ignore_parent() {
            self.ignore_parent = Some(source);
        }
    }

    pub fn binary(&self) -> bool {
        self.binary.unwrap_or(false)
    }

    pub fn ignore_root(&self) -> Option<&std::path::Path> {
        self.ignore_root.as_deref()
    }

    pub fn ignore_patterns(&self) -> Option<&[String]> {
        self.ignore_patterns.as_deref()
    }

    pub fn ignore_hidden(&self) -> bool {
        self.ignore_hidden.unwrap_or(true)
    }

    pub fn ignore_dot(&self) -> bool {
        self.ignore_dot
            .or_else(|| self.ignore_files)
            .unwrap_or(true)
    }

    pub fn ignore_vcs(&self) -> bool {
        self.ignore_vcs
            .or_else(|| self.ignore_files)
            .unwrap_or(true)
    }

    pub fn ignore_global(&self) -> bool {
        self.ignore_global
            .or_else(|| self.ignore_vcs)
            .or_else(|| self.ignore_files)
            .unwrap_or(true)
    }

    pub fn ignore_parent(&self) -> bool {
        self.ignore_parent
            .or_else(|| self.ignore_files)
            .unwrap_or(true)
    }
}

impl WalkSource for Walk {
    fn binary(&self) -> Option<bool> {
        self.binary
    }

    fn ignore_root(&self) -> Option<&std::path::Path> {
        self.ignore_root.as_deref()
    }

    fn ignore_patterns(&self) -> Option<&[String]> {
        self.ignore_patterns.as_deref()
    }

    fn ignore_hidden(&self) -> Option<bool> {
        self.ignore_hidden
    }

    fn ignore_files(&self) -> Option<bool> {
        self.ignore_files
    }

    fn ignore_dot(&self) -> Option<bool> {
        self.ignore_dot
    }

    fn ignore_vcs(&self) -> Option<bool> {
        self.ignore_vcs
    }

    fn ignore_global(&self) -> Option<bool> {
        self.ignore_global
    }

    fn ignore_parent(&self) -> Option<bool> {
        self.ignore_parent
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct FileConfig {
    pub check_filename: Option<bool>,
    pub check_file: Option<bool>,
    pub ignore_hex: Option<bool>,
    pub identifier_leading_digits: Option<bool>,
    pub identifier_leading_chars: Option<String>,
    pub identifier_include_digits: Option<bool>,
    pub identifier_include_chars: Option<String>,
    pub locale: Option<Locale>,
    pub extend_valid_identifiers: Vec<String>,
    pub extend_valid_words: Vec<String>,
}

impl FileConfig {
    pub fn update(&mut self, source: &dyn FileSource) {
        if let Some(source) = source.check_filename() {
            self.check_filename = Some(source);
        }
        if let Some(source) = source.check_file() {
            self.check_file = Some(source);
        }
        if let Some(source) = source.ignore_hex() {
            self.ignore_hex = Some(source);
        }
        if let Some(source) = source.identifier_leading_digits() {
            self.identifier_leading_digits = Some(source);
        }
        if let Some(source) = source.identifier_leading_chars() {
            self.identifier_leading_chars = Some(source.to_owned());
        }
        if let Some(source) = source.identifier_include_digits() {
            self.identifier_include_digits = Some(source);
        }
        if let Some(source) = source.identifier_include_chars() {
            self.identifier_include_chars = Some(source.to_owned());
        }
        if let Some(source) = source.locale() {
            self.locale = Some(source);
        }
        self.extend_valid_identifiers
            .extend(source.extend_valid_identifiers().iter().cloned());
        self.extend_valid_words
            .extend(source.extend_valid_words().iter().cloned());
    }

    pub fn check_filename(&self) -> bool {
        self.check_filename.unwrap_or(true)
    }

    pub fn check_file(&self) -> bool {
        self.check_file.unwrap_or(true)
    }

    pub fn ignore_hex(&self) -> bool {
        self.ignore_hex.unwrap_or(true)
    }

    pub fn identifier_leading_digits(&self) -> bool {
        self.identifier_leading_digits.unwrap_or(false)
    }

    pub fn identifier_leading_chars(&self) -> &str {
        self.identifier_leading_chars.as_deref().unwrap_or("_")
    }

    pub fn identifier_include_digits(&self) -> bool {
        self.identifier_include_digits.unwrap_or(true)
    }

    pub fn identifier_include_chars(&self) -> &str {
        self.identifier_include_chars.as_deref().unwrap_or("_'")
    }

    pub fn locale(&self) -> Locale {
        self.locale.unwrap_or_default()
    }

    pub fn extend_valid_identifiers(&self) -> &[String] {
        self.extend_valid_identifiers.as_slice()
    }

    pub fn extend_valid_words(&self) -> &[String] {
        self.extend_valid_words.as_slice()
    }
}

impl FileSource for FileConfig {
    fn check_filename(&self) -> Option<bool> {
        self.check_filename
    }

    fn check_file(&self) -> Option<bool> {
        self.check_file
    }

    fn ignore_hex(&self) -> Option<bool> {
        self.ignore_hex
    }

    fn identifier_leading_digits(&self) -> Option<bool> {
        self.identifier_leading_digits
    }

    fn identifier_leading_chars(&self) -> Option<&str> {
        self.identifier_leading_chars.as_deref()
    }

    fn identifier_include_digits(&self) -> Option<bool> {
        self.identifier_include_digits
    }

    fn identifier_include_chars(&self) -> Option<&str> {
        self.identifier_include_chars.as_deref()
    }

    fn locale(&self) -> Option<Locale> {
        self.locale
    }

    fn extend_valid_identifiers(&self) -> &[String] {
        self.extend_valid_identifiers.as_slice()
    }

    fn extend_valid_words(&self) -> &[String] {
        self.extend_valid_words.as_slice()
    }
}

fn find_project_file(dir: std::path::PathBuf, name: &str) -> Option<std::path::PathBuf> {
    let mut file_path = dir;
    file_path.push(name);
    while !file_path.exists() {
        file_path.pop(); // filename
        let hit_bottom = !file_path.pop();
        if hit_bottom {
            return None;
        }
        file_path.push(name);
    }
    Some(file_path)
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Locale {
    En,
    EnUs,
    EnGb,
    EnCa,
    EnAu,
}

impl Locale {
    pub fn category(self) -> Option<typos_vars::Category> {
        match self {
            Locale::En => None,
            Locale::EnUs => Some(typos_vars::Category::American),
            Locale::EnGb => Some(typos_vars::Category::BritishIse),
            Locale::EnCa => Some(typos_vars::Category::Canadian),
            Locale::EnAu => Some(typos_vars::Category::Australian),
        }
    }

    pub fn variants() -> [&'static str; 5] {
        ["en", "en-us", "en-gb", "en-ca", "en-au"]
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::En
    }
}

impl std::str::FromStr for Locale {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "en" => Ok(Locale::En),
            "en-us" => Ok(Locale::EnUs),
            "en-gb" => Ok(Locale::EnGb),
            "en-ca" => Ok(Locale::EnCa),
            "en-au" => Ok(Locale::EnAu),
            _ => Err("valid values: en, en-us, en-gb, en-ca, en-au".to_owned()),
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Locale::En => write!(f, "en"),
            Locale::EnUs => write!(f, "en-us"),
            Locale::EnGb => write!(f, "en-gb"),
            Locale::EnCa => write!(f, "en-ca"),
            Locale::EnAu => write!(f, "en-au"),
        }
    }
}
