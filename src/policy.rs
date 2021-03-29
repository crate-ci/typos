pub struct ConfigStorage {
    arena: std::sync::Mutex<typed_arena::Arena<kstring::KString>>,
}

impl ConfigStorage {
    pub fn new() -> Self {
        Self {
            arena: std::sync::Mutex::new(typed_arena::Arena::new()),
        }
    }

    fn get<'s>(&'s self, other: &str) -> &'s str {
        // Safe because we the references are stable once created.
        //
        // Trying to get this handled inside of `typed_arena` directly, see
        // https://github.com/SimonSapin/rust-typed-arena/issues/49#issuecomment-809517312
        unsafe {
            std::mem::transmute::<&str, &str>(
                self.arena
                    .lock()
                    .unwrap()
                    .alloc(kstring::KString::from_ref(other)),
            )
        }
    }
}

pub struct ConfigEngine<'s> {
    storage: &'s ConfigStorage,

    overrides: Option<crate::config::EngineConfig>,
    custom: Option<crate::config::Config>,
    isolated: bool,

    configs: std::collections::HashMap<std::path::PathBuf, DirConfig>,
    files: Intern<crate::config::Walk>,
    tokenizer: Intern<typos::tokens::Tokenizer>,
    dict: Intern<crate::dict::Override<'s, 's, crate::dict::BuiltIn>>,
}

impl<'s> ConfigEngine<'s> {
    pub fn new(storage: &'s ConfigStorage) -> Self {
        Self {
            storage,
            overrides: Default::default(),
            custom: Default::default(),
            configs: Default::default(),
            isolated: false,
            files: Default::default(),
            tokenizer: Default::default(),
            dict: Default::default(),
        }
    }

    pub fn set_overrides(&mut self, overrides: crate::config::EngineConfig) -> &mut Self {
        self.overrides = Some(overrides);
        self
    }

    pub fn set_custom_config(&mut self, custom: crate::config::Config) -> &mut Self {
        self.custom = Some(custom);
        self
    }

    pub fn set_isolated(&mut self, isolated: bool) -> &mut Self {
        self.isolated = isolated;
        self
    }

    pub fn files(&mut self, cwd: &std::path::Path) -> &crate::config::Walk {
        let dir = self
            .configs
            .get(cwd)
            .expect("`init_dir` must be called first");
        self.get_files(dir)
    }

    pub fn policy(&self, path: &std::path::Path) -> Policy<'_, '_> {
        let dir = self
            .get_dir(path)
            .expect("`files()` should be called first");
        Policy {
            check_filenames: dir.check_filenames,
            check_files: dir.check_files,
            binary: dir.binary,
            tokenizer: self.get_tokenizer(dir),
            dict: self.get_dict(dir),
        }
    }

    fn get_files(&self, dir: &DirConfig) -> &crate::config::Walk {
        self.files.get(dir.files)
    }

    fn get_tokenizer(&self, dir: &DirConfig) -> &typos::tokens::Tokenizer {
        self.tokenizer.get(dir.tokenizer)
    }

    fn get_dict(&self, dir: &DirConfig) -> &dyn typos::Dictionary {
        self.dict.get(dir.dict)
    }

    fn get_dir(&self, path: &std::path::Path) -> Option<&DirConfig> {
        for path in path.ancestors() {
            if let Some(dir) = self.configs.get(path) {
                return Some(dir);
            }
        }
        None
    }

    pub fn load_config(
        &self,
        cwd: &std::path::Path,
    ) -> Result<crate::config::Config, anyhow::Error> {
        let mut config = crate::config::Config::default();

        if !self.isolated {
            if let Some(derived) = crate::config::Config::from_dir(cwd)? {
                config.update(&derived);
            }
        }
        if let Some(custom) = self.custom.as_ref() {
            config.update(custom);
        }
        if let Some(overrides) = self.overrides.as_ref() {
            config.default.update(overrides);
        }

        Ok(config)
    }

    pub fn init_dir(&mut self, cwd: &std::path::Path) -> Result<(), anyhow::Error> {
        if self.configs.contains_key(cwd) {
            return Ok(());
        }

        let config = self.load_config(cwd)?;

        let crate::config::Config { files, default } = config;
        let binary = default.binary();
        let check_filename = default.check_filename();
        let check_file = default.check_file();
        let crate::config::EngineConfig {
            tokenizer, dict, ..
        } = default;
        let tokenizer_config =
            tokenizer.unwrap_or_else(|| crate::config::TokenizerConfig::from_defaults());
        let dict_config = dict.unwrap_or_else(|| crate::config::DictConfig::from_defaults());

        let tokenizer = typos::tokens::TokenizerBuilder::new()
            .ignore_hex(tokenizer_config.ignore_hex())
            .leading_digits(tokenizer_config.identifier_leading_digits())
            .leading_chars(tokenizer_config.identifier_leading_chars().to_owned())
            .include_digits(tokenizer_config.identifier_include_digits())
            .include_chars(tokenizer_config.identifier_include_chars().to_owned())
            .build();

        let dict = crate::dict::BuiltIn::new(dict_config.locale());
        let mut dict = crate::dict::Override::new(dict);
        dict.identifiers(
            dict_config
                .extend_identifiers()
                .map(|(k, v)| (self.storage.get(k), self.storage.get(v))),
        );
        dict.words(
            dict_config
                .extend_words()
                .map(|(k, v)| (self.storage.get(k), self.storage.get(v))),
        );

        let dict = self.dict.intern(dict);
        let files = self.files.intern(files);
        let tokenizer = self.tokenizer.intern(tokenizer);

        let dir = DirConfig {
            files,
            check_filenames: check_filename,
            check_files: check_file,
            binary: binary,
            tokenizer,
            dict,
        };

        self.configs.insert(cwd.to_owned(), dir);
        Ok(())
    }
}

struct Intern<T> {
    data: Vec<T>,
}

impl<T> Intern<T> {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn intern(&mut self, value: T) -> usize {
        let symbol = self.data.len();
        self.data.push(value);
        symbol
    }

    pub fn get(&self, symbol: usize) -> &T {
        &self.data[symbol]
    }
}

impl<T> Default for Intern<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct DirConfig {
    files: usize,
    tokenizer: usize,
    dict: usize,
    check_filenames: bool,
    check_files: bool,
    binary: bool,
}

#[non_exhaustive]
#[derive(derive_setters::Setters)]
pub struct Policy<'t, 'd> {
    pub check_filenames: bool,
    pub check_files: bool,
    pub binary: bool,
    pub tokenizer: &'t typos::tokens::Tokenizer,
    pub dict: &'d dyn typos::Dictionary,
}

impl<'t, 'd> Policy<'t, 'd> {
    pub fn new() -> Self {
        Default::default()
    }
}

static DEFAULT_TOKENIZER: once_cell::sync::Lazy<typos::tokens::Tokenizer> =
    once_cell::sync::Lazy::new(typos::tokens::Tokenizer::new);
static DEFAULT_DICT: crate::dict::BuiltIn = crate::dict::BuiltIn::new(crate::config::Locale::En);

impl<'t, 'd> Default for Policy<'t, 'd> {
    fn default() -> Self {
        Self {
            check_filenames: true,
            check_files: true,
            binary: false,
            tokenizer: &DEFAULT_TOKENIZER,
            dict: &DEFAULT_DICT,
        }
    }
}
