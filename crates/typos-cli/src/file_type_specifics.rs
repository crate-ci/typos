//! This module specifies [`EngineConfig`] defaults for the file types defined in [`default_types`].
//!
//! [`EngineConfig`]: crate::config::EngineConfig
//! [`default_types`]: crate::default_types

/// Set `check_file` to `false` for these types.
pub const NO_CHECK_TYPES: &[&str] = &["cert", "lock"];

pub const TYPE_SPECIFIC_DICTS: &[(&str, StaticDictConfig)] = &[
    (
        "css",
        StaticDictConfig {
            ignore_idents: &[
                "nd", // CSS class used by pygments (see https://github.com/pygments/pygments/blob/2.16.1/pygments/token.py#L146)
            ],
            ignore_words: &[],
        },
    ),
    (
        "man",
        StaticDictConfig {
            ignore_idents: &[
                "Nd", // .Nd macro of mdoc (see https://man.openbsd.org/mdoc.7#Nd)
            ],
            ignore_words: &[],
        },
    ),
    (
        "py",
        StaticDictConfig {
            ignore_idents: &[
                "NDArray", // numpy.typing.NDArray
            ],
            ignore_words: &[],
        },
    ),
    (
        "rust",
        StaticDictConfig {
            ignore_idents: &[
                "flate2", // https://crates.io/crates/flate2
            ],
            ignore_words: &[
                "ser", // serde::ser, serde_json::ser, etc.
            ],
        },
    ),
    (
        "sh",
        StaticDictConfig {
            ignore_idents: &[
                "ot", // the test command from GNU coreutils supports an -ot argument (see https://www.gnu.org/software/coreutils/manual/html_node/File-characteristic-tests.html)
                "stap", // command from SystemTap (see https://sourceware.org/systemtap/man/stap.1.html)
            ],
            ignore_words: &[],
        },
    ),
    (
        "vim",
        StaticDictConfig {
            ignore_idents: &[
                "windo", // https://vimdoc.sourceforge.net/htmldoc/windows.html#:windo
            ],
            ignore_words: &[],
        },
    ),
    (
        "vimscript",
        StaticDictConfig {
            ignore_idents: &[
                "windo", // https://vimdoc.sourceforge.net/htmldoc/windows.html#:windo
            ],
            ignore_words: &[],
        },
    ),
];

pub struct StaticDictConfig {
    pub ignore_idents: &'static [&'static str],
    pub ignore_words: &'static [&'static str],
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::TYPE_SPECIFIC_DICTS;

    #[test]
    fn test_type_specific_dicts_contains_no_duplicates() {
        let types: Vec<_> = TYPE_SPECIFIC_DICTS.iter().map(|(typ, _)| *typ).collect();
        let types_unique: Vec<_> = types.clone().into_iter().unique().collect();

        snapbox::assert_eq(types.join("\n"), types_unique.join("\n"));
    }

    #[test]
    fn test_type_specific_dicts_is_sorted() {
        // The order of the entries in TYPE_SPECIFIC_DICTS actually doesn't
        // affect the runtime behavior, we just want them ordered
        // so that it's easier to find entries for contributors.

        let types: Vec<_> = TYPE_SPECIFIC_DICTS.iter().map(|(typ, _)| *typ).collect();
        let types_sorted: Vec<_> = types.iter().cloned().sorted().collect();

        snapbox::assert_eq(types.join("\n"), types_sorted.join("\n"));
    }
}
