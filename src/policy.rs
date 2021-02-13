pub struct ConfigStorage {
    arena: typed_arena::Arena<kstring::KString>,
}

impl ConfigStorage {
    pub fn new() -> Self {
        Self {
            arena: typed_arena::Arena::new(),
        }
    }

    fn get<'s>(&'s self, other: &str) -> &'s str {
        self.arena.alloc(kstring::KString::from_ref(other))
    }
}

pub struct ConfigEngine<'s> {
    files: crate::config::Walk,
    check_filenames: bool,
    check_files: bool,
    binary: bool,
    tokenizer: typos::tokens::Tokenizer,
    dictionary: crate::dict::Override<'s, 's, crate::dict::BuiltIn>,
}

impl<'s> ConfigEngine<'s> {
    pub fn new(config: crate::config::Config, storage: &'s ConfigStorage) -> Self {
        let crate::config::Config { files, default } = config;

        let tokenizer = typos::tokens::TokenizerBuilder::new()
            .ignore_hex(default.ignore_hex())
            .leading_digits(default.identifier_leading_digits())
            .leading_chars(default.identifier_leading_chars().to_owned())
            .include_digits(default.identifier_include_digits())
            .include_chars(default.identifier_include_chars().to_owned())
            .build();

        let dictionary = crate::dict::BuiltIn::new(default.locale());
        let mut dictionary = crate::dict::Override::new(dictionary);
        dictionary.identifiers(
            default
                .extend_identifiers()
                .map(|(k, v)| (storage.get(k), storage.get(v))),
        );
        dictionary.words(
            default
                .extend_words()
                .map(|(k, v)| (storage.get(k), storage.get(v))),
        );

        Self {
            files,
            check_filenames: default.check_filename(),
            check_files: default.check_file(),
            binary: default.binary(),
            tokenizer,
            dictionary,
        }
    }

    pub fn files(&self) -> &crate::config::Walk {
        &self.files
    }

    pub fn policy(&self) -> Policy<'_, '_> {
        Policy {
            check_filenames: self.check_filenames,
            check_files: self.check_files,
            binary: self.binary,
            tokenizer: &self.tokenizer,
            dictionary: &self.dictionary,
        }
    }
}

#[non_exhaustive]
#[derive(derive_setters::Setters)]
pub struct Policy<'t, 'd> {
    pub check_filenames: bool,
    pub check_files: bool,
    pub binary: bool,
    pub tokenizer: &'t typos::tokens::Tokenizer,
    pub dictionary: &'d dyn typos::Dictionary,
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
            dictionary: &DEFAULT_DICT,
        }
    }
}
