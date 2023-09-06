use std::collections::HashMap;

use kstring::KString;

use crate::file_type_specifics;

pub const SUPPORTED_FILE_NAMES: &[&str] =
    &["typos.toml", "_typos.toml", ".typos.toml", "pyproject.toml"];

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub files: Walk,
    pub default: EngineConfig,
    #[serde(rename = "type")]
    pub type_: TypeEngineConfig,
    #[serde(skip)]
    pub overrides: EngineConfig,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct PyprojectTomlConfig {
    pub tool: PyprojectTomlTool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct PyprojectTomlTool {
    pub typos: Option<Config>,
}

impl Config {
    pub fn from_dir(cwd: &std::path::Path) -> Result<Option<Self>, anyhow::Error> {
        for file in find_project_files(cwd, SUPPORTED_FILE_NAMES) {
            log::debug!("Loading {}", file.display());
            if let Some(config) = Self::from_file(&file)? {
                return Ok(Some(config));
            }
        }

        Ok(None)
    }

    pub fn from_file(path: &std::path::Path) -> Result<Option<Self>, anyhow::Error> {
        let s = std::fs::read_to_string(path).map_err(|err| {
            let kind = err.kind();
            std::io::Error::new(
                kind,
                format!("could not read config at `{}`", path.display()),
            )
        })?;

        if path.file_name().unwrap() == "pyproject.toml" {
            let config = toml::from_str::<PyprojectTomlConfig>(&s)?;

            if config.tool.typos.is_none() {
                log::debug!("No `tool.typos` section found in `pyproject.toml`, skipping");

                Ok(None)
            } else {
                Ok(config.tool.typos)
            }
        } else {
            Self::from_toml(&s).map(Some)
        }
    }

    pub fn from_toml(data: &str) -> Result<Self, anyhow::Error> {
        let content = toml::from_str(data)?;
        Ok(content)
    }

    pub fn from_defaults() -> Self {
        Self {
            files: Walk::from_defaults(),
            default: EngineConfig::from_defaults(),
            type_: TypeEngineConfig::from_defaults(),
            overrides: EngineConfig::default(),
        }
    }

    pub fn update(&mut self, source: &Config) {
        self.files.update(&source.files);
        self.default.update(&source.default);
        self.type_.update(&source.type_);
        self.overrides.update(&source.overrides);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Walk {
    pub extend_exclude: Vec<String>,
    /// Skip hidden files and directories.
    pub ignore_hidden: Option<bool>,
    /// Respect ignore files.
    pub ignore_files: Option<bool>,
    /// Respect .ignore files.
    pub ignore_dot: Option<bool>,
    /// Respect ignore files in vcs directories.
    pub ignore_vcs: Option<bool>,
    /// Respect global ignore files.
    pub ignore_global: Option<bool>,
    /// Respect ignore files in parent directories.
    pub ignore_parent: Option<bool>,
}

impl Walk {
    pub fn from_defaults() -> Self {
        let empty = Self::default();
        Self {
            extend_exclude: empty.extend_exclude.clone(),
            ignore_hidden: Some(empty.ignore_hidden()),
            ignore_files: Some(true),
            ignore_dot: Some(empty.ignore_dot()),
            ignore_vcs: Some(empty.ignore_vcs()),
            ignore_global: Some(empty.ignore_global()),
            ignore_parent: Some(empty.ignore_parent()),
        }
    }

    pub fn update(&mut self, source: &Walk) {
        self.extend_exclude
            .extend(source.extend_exclude.iter().cloned());
        if let Some(source) = source.ignore_hidden {
            self.ignore_hidden = Some(source);
        }
        if let Some(source) = source.ignore_files {
            self.ignore_files = Some(source);
            self.ignore_dot = None;
            self.ignore_vcs = None;
            self.ignore_global = None;
            self.ignore_parent = None;
        }
        if let Some(source) = source.ignore_dot {
            self.ignore_dot = Some(source);
        }
        if let Some(source) = source.ignore_vcs {
            self.ignore_vcs = Some(source);
            self.ignore_global = None;
        }
        if let Some(source) = source.ignore_global {
            self.ignore_global = Some(source);
        }
        if let Some(source) = source.ignore_parent {
            self.ignore_parent = Some(source);
        }
    }

    pub fn extend_exclude(&self) -> &[String] {
        &self.extend_exclude
    }

    pub fn ignore_hidden(&self) -> bool {
        self.ignore_hidden.unwrap_or(true)
    }

    pub fn ignore_dot(&self) -> bool {
        self.ignore_dot.or(self.ignore_files).unwrap_or(true)
    }

    pub fn ignore_vcs(&self) -> bool {
        self.ignore_vcs.or(self.ignore_files).unwrap_or(true)
    }

    pub fn ignore_global(&self) -> bool {
        self.ignore_global
            .or(self.ignore_vcs)
            .or(self.ignore_files)
            .unwrap_or(true)
    }

    pub fn ignore_parent(&self) -> bool {
        self.ignore_parent.or(self.ignore_files).unwrap_or(true)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[serde(transparent)]
pub struct TypeEngineConfig {
    pub patterns: std::collections::HashMap<KString, GlobEngineConfig>,
}

impl TypeEngineConfig {
    pub fn from_defaults() -> Self {
        let mut patterns = HashMap::new();

        for no_check_type in file_type_specifics::NO_CHECK_TYPES {
            patterns.insert(
                KString::from(*no_check_type),
                GlobEngineConfig {
                    extend_glob: Vec::new(),
                    engine: EngineConfig {
                        check_file: Some(false),
                        ..Default::default()
                    },
                },
            );
        }

        for (typ, dict_config) in file_type_specifics::TYPE_SPECIFIC_DICTS {
            patterns.insert(
                KString::from(*typ),
                GlobEngineConfig {
                    extend_glob: Vec::new(),
                    engine: EngineConfig {
                        dict: Some(DictConfig {
                            extend_identifiers: dict_config
                                .ignore_idents
                                .iter()
                                .map(|key| ((*key).into(), (*key).into()))
                                .collect(),
                            extend_words: dict_config
                                .ignore_words
                                .iter()
                                .map(|key| ((*key).into(), (*key).into()))
                                .collect(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                },
            );
        }

        Self { patterns }
    }

    pub fn update(&mut self, source: &Self) {
        for (type_name, engine) in source.patterns.iter() {
            self.patterns
                .entry(type_name.to_owned())
                .or_insert_with(GlobEngineConfig::default)
                .update(engine);
        }
    }

    pub fn patterns(&self) -> impl Iterator<Item = (kstring::KString, GlobEngineConfig)> {
        let mut engine = Self::from_defaults();
        engine.update(self);
        engine.patterns.into_iter()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
//#[serde(deny_unknown_fields)]  // Doesn't work with `flatten`
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct GlobEngineConfig {
    pub extend_glob: Vec<kstring::KString>,
    #[serde(flatten)]
    pub engine: EngineConfig,
}

impl GlobEngineConfig {
    pub fn update(&mut self, source: &GlobEngineConfig) {
        self.extend_glob.extend(source.extend_glob.iter().cloned());
        self.engine.update(&source.engine);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
//#[serde(deny_unknown_fields)]  // Doesn't work with `flatten`
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct EngineConfig {
    /// Check binary files.
    pub binary: Option<bool>,
    /// Verifying spelling in file names.
    pub check_filename: Option<bool>,
    /// Verifying spelling in files.
    pub check_file: Option<bool>,
    #[serde(flatten)]
    pub tokenizer: Option<TokenizerConfig>,
    #[serde(flatten)]
    pub dict: Option<DictConfig>,
    #[serde(with = "serde_regex")]
    pub extend_ignore_re: Vec<regex::Regex>,
}

impl EngineConfig {
    pub fn from_defaults() -> Self {
        let empty = Self::default();
        EngineConfig {
            binary: Some(empty.binary()),
            check_filename: Some(empty.check_filename()),
            check_file: Some(empty.check_file()),
            tokenizer: Some(
                empty
                    .tokenizer
                    .unwrap_or_else(TokenizerConfig::from_defaults),
            ),
            dict: Some(empty.dict.unwrap_or_else(DictConfig::from_defaults)),
            extend_ignore_re: Default::default(),
        }
    }

    pub fn update(&mut self, source: &EngineConfig) {
        if let Some(source) = source.binary {
            self.binary = Some(source);
        }
        if let Some(source) = source.check_filename {
            self.check_filename = Some(source);
        }
        if let Some(source) = source.check_file {
            self.check_file = Some(source);
        }
        if let Some(source) = source.tokenizer.as_ref() {
            let mut tokenizer = None;
            std::mem::swap(&mut tokenizer, &mut self.tokenizer);
            let mut tokenizer = tokenizer.unwrap_or_default();
            tokenizer.update(source);
            let mut tokenizer = Some(tokenizer);
            std::mem::swap(&mut tokenizer, &mut self.tokenizer);
        }
        if let Some(source) = source.dict.as_ref() {
            let mut dict = None;
            std::mem::swap(&mut dict, &mut self.dict);
            let mut dict = dict.unwrap_or_default();
            dict.update(source);
            let mut dict = Some(dict);
            std::mem::swap(&mut dict, &mut self.dict);
        }
        self.extend_ignore_re
            .extend(source.extend_ignore_re.iter().cloned());
    }

    pub fn binary(&self) -> bool {
        self.binary.unwrap_or(false)
    }

    pub fn check_filename(&self) -> bool {
        self.check_filename.unwrap_or(true)
    }

    pub fn check_file(&self) -> bool {
        self.check_file.unwrap_or(true)
    }

    pub fn extend_ignore_re(&self) -> Box<dyn Iterator<Item = &regex::Regex> + '_> {
        Box::new(self.extend_ignore_re.iter())
    }
}

impl PartialEq for EngineConfig {
    fn eq(&self, rhs: &Self) -> bool {
        self.binary == rhs.binary
            && self.check_filename == rhs.check_filename
            && self.check_file == rhs.check_file
            && self.tokenizer == rhs.tokenizer
            && self.dict == rhs.dict
            && self
                .extend_ignore_re
                .iter()
                .map(|r| r.as_str())
                .eq(rhs.extend_ignore_re.iter().map(|r| r.as_str()))
    }
}

impl Eq for EngineConfig {}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct TokenizerConfig {
    /// Allow unicode characters in identifiers (and not just ASCII)
    pub unicode: Option<bool>,
    /// Do not check identifiers that appear to be hexadecimal values.
    pub ignore_hex: Option<bool>,
    /// Allow identifiers to start with digits, in addition to letters.
    pub identifier_leading_digits: Option<bool>,
}

impl TokenizerConfig {
    pub fn from_defaults() -> Self {
        let empty = Self::default();
        Self {
            unicode: Some(empty.unicode()),
            ignore_hex: Some(empty.ignore_hex()),
            identifier_leading_digits: Some(empty.identifier_leading_digits()),
        }
    }

    pub fn update(&mut self, source: &TokenizerConfig) {
        if let Some(source) = source.unicode {
            self.unicode = Some(source);
        }
        if let Some(source) = source.ignore_hex {
            self.ignore_hex = Some(source);
        }
        if let Some(source) = source.identifier_leading_digits {
            self.identifier_leading_digits = Some(source);
        }
    }

    pub fn unicode(&self) -> bool {
        self.unicode.unwrap_or(true)
    }

    pub fn ignore_hex(&self) -> bool {
        self.ignore_hex.unwrap_or(true)
    }

    pub fn identifier_leading_digits(&self) -> bool {
        self.identifier_leading_digits.unwrap_or(false)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct DictConfig {
    pub locale: Option<Locale>,
    #[serde(with = "serde_regex")]
    pub extend_ignore_identifiers_re: Vec<regex::Regex>,
    pub extend_identifiers: HashMap<kstring::KString, kstring::KString>,
    pub extend_words: HashMap<kstring::KString, kstring::KString>,
}

impl DictConfig {
    pub fn from_defaults() -> Self {
        let empty = Self::default();
        Self {
            locale: Some(empty.locale()),
            extend_ignore_identifiers_re: Default::default(),
            extend_identifiers: Default::default(),
            extend_words: Default::default(),
        }
    }

    pub fn update(&mut self, source: &DictConfig) {
        if let Some(source) = source.locale {
            self.locale = Some(source);
        }
        self.extend_ignore_identifiers_re
            .extend(source.extend_ignore_identifiers_re.iter().cloned());
        self.extend_identifiers.extend(
            source
                .extend_identifiers
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
        self.extend_words.extend(
            source
                .extend_words
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }

    pub fn locale(&self) -> Locale {
        self.locale.unwrap_or_default()
    }

    pub fn extend_ignore_identifiers_re(&self) -> Box<dyn Iterator<Item = &regex::Regex> + '_> {
        Box::new(self.extend_ignore_identifiers_re.iter())
    }

    pub fn extend_identifiers(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        Box::new(
            self.extend_identifiers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str())),
        )
    }

    pub fn extend_words(&self) -> Box<dyn Iterator<Item = (&str, &str)> + '_> {
        Box::new(
            self.extend_words
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str())),
        )
    }
}

fn find_project_files<'a>(
    dir: &'a std::path::Path,
    names: &'a [&'a str],
) -> impl Iterator<Item = std::path::PathBuf> + 'a {
    names
        .iter()
        .map(|name| dir.join(name))
        .filter(|path| path.exists())
}

impl PartialEq for DictConfig {
    fn eq(&self, rhs: &Self) -> bool {
        self.locale == rhs.locale
            && self
                .extend_ignore_identifiers_re
                .iter()
                .map(|r| r.as_str())
                .eq(rhs.extend_ignore_identifiers_re.iter().map(|r| r.as_str()))
            && self.extend_identifiers == rhs.extend_identifiers
            && self.extend_words == rhs.extend_words
    }
}

impl Eq for DictConfig {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Default)]
pub enum Locale {
    #[default]
    En,
    EnUs,
    EnGb,
    EnCa,
    EnAu,
}

impl Locale {
    pub const fn category(self) -> Option<varcon_core::Category> {
        match self {
            Locale::En => None,
            Locale::EnUs => Some(varcon_core::Category::American),
            Locale::EnGb => Some(varcon_core::Category::BritishIse),
            Locale::EnCa => Some(varcon_core::Category::Canadian),
            Locale::EnAu => Some(varcon_core::Category::Australian),
        }
    }

    pub const fn variants() -> [&'static str; 5] {
        ["en", "en-us", "en-gb", "en-ca", "en-au"]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_defaults() {
        let null = Config::default();
        let defaulted = Config::from_defaults();
        assert_ne!(defaulted, null);
        assert_ne!(defaulted.files, null.files);
        assert_ne!(defaulted.default, null.default);
        assert_ne!(defaulted.default.tokenizer, null.default.tokenizer);
        assert_ne!(defaulted.default.dict, null.default.dict);
    }

    #[test]
    fn test_update_from_nothing() {
        let null = Config::default();
        let defaulted = Config::from_defaults();

        let mut actual = defaulted.clone();
        actual.update(&null);

        assert_eq!(actual, defaulted);
    }

    #[test]
    fn test_update_from_defaults() {
        let null = Config::default();
        let defaulted = Config::from_defaults();

        let mut actual = null;
        actual.update(&defaulted);

        assert_eq!(actual, defaulted);
    }

    #[test]
    fn test_extend_glob_updates() {
        let null = GlobEngineConfig::default();
        let extended = GlobEngineConfig {
            extend_glob: vec!["*.foo".into()],
            ..Default::default()
        };

        let mut actual = null;
        actual.update(&extended);

        assert_eq!(actual, extended);
    }

    #[test]
    fn test_extend_glob_extends() {
        let base = GlobEngineConfig {
            extend_glob: vec!["*.foo".into()],
            ..Default::default()
        };
        let extended = GlobEngineConfig {
            extend_glob: vec!["*.bar".into()],
            ..Default::default()
        };

        let mut actual = base;
        actual.update(&extended);

        let expected: Vec<kstring::KString> = vec!["*.foo".into(), "*.bar".into()];
        assert_eq!(actual.extend_glob, expected);
    }

    #[test]
    fn parse_extend_globs() {
        let input = r#"[type.po]
extend-glob = ["*.po"]
check-file = true
"#;
        let mut expected = Config::default();
        expected.type_.patterns.insert(
            "po".into(),
            GlobEngineConfig {
                extend_glob: vec!["*.po".into()],
                engine: EngineConfig {
                    tokenizer: Some(TokenizerConfig::default()),
                    dict: Some(DictConfig::default()),
                    check_file: Some(true),
                    ..Default::default()
                },
            },
        );
        let actual = Config::from_toml(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_extend_words() {
        let input = r#"[type.shaders]
extend-glob = [
  '*.shader',
  '*.cginc',
]

[type.shaders.extend-words]
inout = "inout"
"#;
        let mut expected = Config::default();
        expected.type_.patterns.insert(
            "shaders".into(),
            GlobEngineConfig {
                extend_glob: vec!["*.shader".into(), "*.cginc".into()],
                engine: EngineConfig {
                    tokenizer: Some(TokenizerConfig::default()),
                    dict: Some(DictConfig {
                        extend_words: maplit::hashmap! {
                            "inout".into() => "inout".into(),
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
        );
        let actual = Config::from_toml(input).unwrap();
        assert_eq!(actual, expected);
    }
}
