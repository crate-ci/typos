use std::io::Read;

pub trait ConfigSource {
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub ignore_hidden: Option<bool>,
    pub ignore_files: Option<bool>,
    pub ignore_dot: Option<bool>,
    pub ignore_vcs: Option<bool>,
    pub ignore_global: Option<bool>,
    pub ignore_parent: Option<bool>,
}

impl Config {
    pub fn from_file(path: &std::path::Path) -> Result<Self, failure::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Self::from_toml(&s)
    }

    pub fn from_toml(data: &str) -> Result<Self, failure::Error> {
        let content = toml::from_str(data)?;
        Ok(content)
    }

    pub fn derive(cwd: &std::path::Path) -> Result<Self, failure::Error> {
        if let Some(path) = find_project_file(cwd.to_owned(), "typos.toml") {
            Self::from_file(&path)
        } else {
            Ok(Default::default())
        }
    }

    pub fn update(&mut self, source: &dyn ConfigSource) {
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

impl ConfigSource for Config {
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
