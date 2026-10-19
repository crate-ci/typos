use std::collections::BTreeMap;
use std::path::Path;

use compact_str::CompactString;

/// Weight of a type from `default_types.rs`; a `[type.*]` table from a config file
/// carries `USER_WEIGHT`, so a user definition wins a glob a built-in type also claims.
const BUILTIN_WEIGHT: usize = 0;
const USER_WEIGHT: usize = 1;

#[derive(Default, Clone, Debug)]
pub(crate) struct TypesBuilder {
    definitions: BTreeMap<CompactString, Vec<(CompactString, usize)>>,
}

impl TypesBuilder {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn add_defaults(&mut self) {
        for (name, globs) in crate::default_types::DEFAULT_TYPES.iter() {
            for glob in globs.iter() {
                self.add_weighted(
                    CompactString::const_new(name),
                    CompactString::const_new(glob),
                    BUILTIN_WEIGHT,
                );
            }
        }
    }

    pub(crate) fn contains_name(&self, name: &str) -> bool {
        self.definitions.contains_key(name)
    }

    pub(crate) fn add(&mut self, name: impl Into<CompactString>, glob: impl Into<CompactString>) {
        self.add_weighted(name, glob, USER_WEIGHT);
    }

    fn add_weighted(
        &mut self,
        name: impl Into<CompactString>,
        glob: impl Into<CompactString>,
        weight: usize,
    ) {
        let name = name.into();
        let glob = glob.into();
        self.definitions
            .entry(name)
            .or_default()
            .push((glob, weight));
    }

    /// Build the matcher.
    ///
    /// Order of `add` does not matter: a file's type is decided by the glob's
    /// specificity, then by the weight of the definition it came from, and two
    /// definitions of the same weight are separated by the type name. Before the
    /// weight was fixed per source, it was the insertion position and two `[type.*]`
    /// tables claiming one glob resolved by the order the config happened to be read
    /// in, so the same file was assigned a different type from run to run.
    pub(crate) fn build(self) -> Result<Types, anyhow::Error> {
        let mut definitions = self
            .definitions
            .iter()
            .flat_map(|(name, globs)| {
                globs.iter().map(move |(glob, weight)| {
                    let sort = sort_key(glob);
                    (sort, weight, name, glob)
                })
            })
            .collect::<Vec<_>>();
        definitions.sort();

        let rev_definitions = definitions
            .iter()
            .map(|(_, _, name, glob)| (*glob, *name))
            .collect::<BTreeMap<_, _>>();
        let mut unique_definitions = BTreeMap::<CompactString, Vec<CompactString>>::new();
        for (glob, name) in rev_definitions {
            unique_definitions
                .entry(name.clone())
                .or_default()
                .push(glob.clone());
        }

        let mut glob_to_name = Vec::new();
        let mut build_set = globset::GlobSetBuilder::new();
        for (_, _, name, glob) in definitions {
            glob_to_name.push(name.clone());
            build_set.add(
                globset::GlobBuilder::new(glob)
                    .literal_separator(true)
                    .build()?,
            );
        }
        let set = build_set.build()?;

        Ok(Types {
            definitions: unique_definitions,
            glob_to_name,
            set,
            matches: std::sync::Arc::new(thread_local::ThreadLocal::default()),
        })
    }
}

fn sort_key(glob: &str) -> Vec<GlobPart<'_>> {
    let mut key = glob
        .split('.')
        .map(|s| {
            if s == "*" {
                GlobPart::Wild(s)
            } else if s.contains('*') {
                GlobPart::PartialWild(s)
            } else {
                GlobPart::Literalish(s)
            }
        })
        .collect::<Vec<_>>();
    key.reverse();
    key
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum GlobPart<'s> {
    Wild(&'s str),
    PartialWild(&'s str),
    Literalish(&'s str),
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Types {
    definitions: BTreeMap<CompactString, Vec<CompactString>>,
    glob_to_name: Vec<CompactString>,
    set: globset::GlobSet,
    /// Temporary storage for globs that match.
    matches: std::sync::Arc<thread_local::ThreadLocal<std::cell::RefCell<Vec<usize>>>>,
}

impl Types {
    pub(crate) fn definitions(&self) -> &BTreeMap<CompactString, Vec<CompactString>> {
        &self.definitions
    }

    pub(crate) fn file_matched(&self, path: &Path) -> Option<&str> {
        let mut mpath = Path::new(path);
        let mut matches = self.matches.get_or_default().borrow_mut();
        loop {
            self.set.matches_into(mpath.file_name()?, &mut matches);
            if !matches.is_empty() {
                break;
            }
            match mpath.extension() {
                None => break,
                Some(ext) => {
                    if ext == "in" {
                        mpath = Path::new(mpath.file_stem()?);
                        continue;
                    }
                }
            }
            break;
        }
        matches
            .last()
            .copied()
            .map(|i| self.glob_to_name[i].as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! matched {
        ($name:ident, $types:expr, $path:expr, $matched:expr) => {
            #[test]
            fn $name() {
                let mut btypes = TypesBuilder::new();
                for (name, globs) in $types {
                    for glob in *globs {
                        btypes.add(*name, *glob);
                    }
                }
                let types = btypes.build().unwrap();
                let actual = types.file_matched(std::path::Path::new($path));
                let expected: Option<&str> = $matched.into();
                assert_eq!(expected, actual, "{}", $path);
            }
        };
    }

    fn types() -> &'static [(&'static str, &'static [&'static str])] {
        &[
            ("html", &["*.html", "*.htm"]),
            ("js", &["*.js"]),
            ("json", &["*.json"]),
            ("lock", &["package-lock.json", "*.lock"]),
            ("js-in", &["*.js.in"]),
        ]
    }
    fn in_types() -> &'static [(&'static str, &'static [&'static str])] {
        &[("html", &["*.html", "*.htm"]), ("in-canary", &["*.in"])]
    }

    matched!(basic_match, types(), "leftpad.js", "js");
    matched!(multi_def_1, types(), "index.html", "html");
    matched!(multi_def_2, types(), "index.htm", "html");
    matched!(no_match, types(), "leftpad.ada", None);
    matched!(more_specific, types(), "package-lock.json", "lock");
    matched!(basic_in, types(), "index.html.in", "html");
    matched!(basic_in_in, types(), "index.html.in.in", "html");
    matched!(ext_plus_in, types(), "foo.js.in", "js-in");
    matched!(toplevel_in, in_types(), "index.html.in", "in-canary");

    macro_rules! sort {
        ($name:ident, $actual:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let expected = $expected.into_iter().collect::<Vec<&str>>();

                let mut actual = $actual.into_iter().collect::<Vec<&str>>();
                actual.sort_by_key(|s| sort_key(s));

                assert_eq!(expected, actual);
            }
        };
    }

    sort!(literal_sort, ["b", "c", "a"], ["a", "b", "c"]);
    sort!(
        basic_glob_sort,
        ["a_specific", "z_partial*"],
        ["z_partial*", "a_specific"]
    );
    sort!(
        nested_glob_sort,
        ["a.specific", "z*.partial", "z.partial*"],
        ["z.partial*", "z*.partial", "a.specific"]
    );
    sort!(most_specific, ["*.txt.in", "*.in"], ["*.in", "*.txt.in"]);

    #[test]
    fn same_glob_in_two_types_resolves_by_name_not_insertion_order() {
        // `[type.prose] extend-glob = ["*.md"]` next to
        // `[type.prose-skip] extend-glob = ["*.md"]`: whichever the config reader
        // yields first must not decide which type the file gets.
        let mut forward = TypesBuilder::new();
        forward.add("prose", "*.md");
        forward.add("prose-skip", "*.md");

        let mut reverse = TypesBuilder::new();
        reverse.add("prose-skip", "*.md");
        reverse.add("prose", "*.md");

        let path = Path::new("README.md");
        assert_eq!(
            Some("prose-skip"),
            forward.build().unwrap().file_matched(path)
        );
        assert_eq!(
            Some("prose-skip"),
            reverse.build().unwrap().file_matched(path)
        );
    }

    #[test]
    fn a_user_type_wins_the_glob_of_a_builtin_type() {
        let mut types = TypesBuilder::new();
        types.add_defaults();
        // `aaa-markdown` sorts before the built-in `md`: the weight of the
        // definition, not the type name, is what makes a `[type.*]` table win here.
        types.add("aaa-markdown", "*.md");

        let types = types.build().unwrap();
        assert_eq!(
            Some("aaa-markdown"),
            types.file_matched(Path::new("README.md"))
        );
    }
}
