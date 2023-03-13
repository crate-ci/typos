use std::collections::BTreeMap;

use kstring::KString;

#[derive(Default, Clone, Debug)]
pub struct TypesBuilder {
    definitions: BTreeMap<KString, Vec<KString>>,
}

impl TypesBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_defaults(&mut self) {
        self.definitions.extend(
            crate::default_types::DEFAULT_TYPES
                .iter()
                .map(|(name, glob)| {
                    let name = KString::from(*name);
                    let globs = glob.iter().map(|s| KString::from(*s)).collect();
                    (name, globs)
                }),
        );
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.definitions.contains_key(name)
    }

    pub fn add(&mut self, name: impl Into<KString>, glob: impl Into<KString>) {
        let name = name.into();
        let glob = glob.into();
        self.definitions.entry(name).or_default().push(glob);
    }

    pub fn build(self) -> Result<Types, anyhow::Error> {
        let mut definitions = self
            .definitions
            .iter()
            .flat_map(|(name, globs)| {
                globs.iter().map(move |glob| {
                    let sort = sort_key(glob);
                    (sort, name, glob)
                })
            })
            .collect::<Vec<_>>();
        definitions.sort();

        let mut glob_to_name = Vec::new();
        let mut build_set = globset::GlobSetBuilder::new();
        for (_, name, glob) in definitions {
            glob_to_name.push(name.clone());
            build_set.add(
                globset::GlobBuilder::new(glob)
                    .literal_separator(true)
                    .build()?,
            );
        }
        let set = build_set.build()?;

        Ok(Types {
            definitions: self.definitions,
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
pub struct Types {
    definitions: BTreeMap<KString, Vec<KString>>,
    glob_to_name: Vec<KString>,
    set: globset::GlobSet,
    /// Temporary storage for globs that match.
    matches: std::sync::Arc<thread_local::ThreadLocal<std::cell::RefCell<Vec<usize>>>>,
}

impl Types {
    pub fn definitions(&self) -> &BTreeMap<KString, Vec<KString>> {
        &self.definitions
    }

    pub fn file_matched(&self, path: &std::path::Path) -> Option<&str> {
        let file_name = path.file_name()?;
        let mut matches = self.matches.get_or_default().borrow_mut();
        self.set.matches_into(file_name, &mut *matches);
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
        ]
    }

    matched!(basic_match, types(), "leftpad.js", "js");
    matched!(multi_def_1, types(), "index.html", "html");
    matched!(multi_def_2, types(), "index.htm", "html");
    matched!(no_match, types(), "leftpad.ada", None);
    matched!(more_specific, types(), "package-lock.json", "lock");

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
}
