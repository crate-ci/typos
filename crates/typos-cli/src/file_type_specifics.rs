//! This module specifies [`EngineConfig`] defaults for the file types defined in [`default_types`].
//!
//! [`EngineConfig`]: crate::config::EngineConfig
//! [`default_types`]: crate::default_types

/// Set `check_file` to `false` for these types.
pub(crate) const NO_CHECK_TYPES: &[&str] = &["cert", "lock"];

pub(crate) const TYPE_SPECIFIC_DICTS: &[(&str, StaticDictConfig)] = &[
    (
        "cpp",
        StaticDictConfig {
            ignore_idents: &[
                "countr_one", // `std::countr_one`
            ],
            ignore_words: &[],
        },
    ),
    (
        "css",
        StaticDictConfig {
            ignore_idents: &[
                "nd", // CSS class used by pygments (see https://github.com/pygments/pygments/blob/2.16.1/pygments/token.py#L146)
                "wdth", // Tag in OpenType 1.8 design-variation axes (see https://github.com/microsoft/OpenTypeDesignVariationAxisTags/blob/5ea229006014c614654242a29f49424c1d0659fa/BackgroundOnAxes.md?plain=1#L25)
            ],
            ignore_words: &[],
        },
    ),
    (
        "go",
        StaticDictConfig {
            ignore_idents: &[
                "flate", // https://pkg.go.dev/compress/flate
            ],
            ignore_words: &[],
        },
    ),
    (
        "html",
        StaticDictConfig {
            ignore_idents: &[
                "wdth", // Tag in OpenType design-variation axes, see "css" section for same entry
            ],
            ignore_words: &[],
        },
    ),
    (
        "jl",
        StaticDictConfig {
            ignore_idents: &[],
            ignore_words: &[
                "egal",  // name for `===` operator
                "egals", // name for `===` operator
                "modul", // stand-in for `module` when needing to avoid the keyword
                "usig",  // stand-in for `using` when needing to avoid the keyword
            ],
        },
    ),
    (
        "less",
        StaticDictConfig {
            ignore_idents: &[
                "wdth", // Tag in OpenType design-variation axes, see "css" section for same entry
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
                "NDArray",  // numpy.typing.NDArray
                "EOFError", // std
                "arange",   // torch.arange, numpy.arange
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
        "sass",
        StaticDictConfig {
            ignore_idents: &[
                "wdth", // Tag in OpenType design-variation axes, see "css" section for same entry
            ],
            ignore_words: &[],
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
        "stylus",
        StaticDictConfig {
            ignore_idents: &[
                "wdth", // Tag in OpenType design-variation axes, see "css" section for same entry
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

pub(crate) struct StaticDictConfig {
    pub(crate) ignore_idents: &'static [&'static str],
    pub(crate) ignore_words: &'static [&'static str],
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use snapbox::prelude::*;

    use super::TYPE_SPECIFIC_DICTS;

    #[test]
    fn test_type_specific_dicts_contains_no_duplicates() {
        let types: Vec<_> = TYPE_SPECIFIC_DICTS.iter().map(|(typ, _)| *typ).collect();
        let types_unique: Vec<_> = types.clone().into_iter().unique().collect();

        snapbox::assert_data_eq!(types_unique.join("\n"), types.join("\n").raw());
    }

    #[test]
    fn test_type_specific_dicts_is_sorted() {
        // The order of the entries in TYPE_SPECIFIC_DICTS actually doesn't
        // affect the runtime behavior, we just want them ordered
        // so that it's easier to find entries for contributors.

        let types: Vec<_> = TYPE_SPECIFIC_DICTS.iter().map(|(typ, _)| *typ).collect();
        let types_sorted: Vec<_> = types.iter().cloned().sorted().collect();

        snapbox::assert_data_eq!(types_sorted.join("\n"), types.join("\n").raw());
    }
}
