use std::collections::BTreeMap;
use std::path::Path;

use kstring::KString;

#[derive(Default, Clone, Debug)]
pub struct TypesBuilder {
    definitions: BTreeMap<KString, Vec<(KString, usize)>>,
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
                    let globs = glob.iter().map(|s| (KString::from(*s), 0)).collect();
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
        let weight = self.definitions.len();
        self.definitions
            .entry(name)
            .or_default()
            .push((glob, weight));
    }

    pub fn build(self) -> Result<Types, anyhow::Error> {
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
        let mut unique_definitions = BTreeMap::<KString, Vec<KString>>::new();
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
}
