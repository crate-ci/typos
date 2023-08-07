use indexmap::IndexSet;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use unicase::UniCase;

type Dict = BTreeMap<UniCase<String>, IndexSet<String>>;

#[test]
fn verify() {
    let asset_path = "assets/words.csv";
    let typos_dict = parse_dict(asset_path);
    let new_dict = process(typos_dict);

    let mut content = vec![];

    let mut wtr = csv::WriterBuilder::new()
        .flexible(true)
        .from_writer(&mut content);
    for (typo, corrections) in new_dict {
        let mut row = vec![typo.as_str().to_owned()];
        row.extend(corrections);
        wtr.write_record(&row).unwrap();
    }
    wtr.flush().unwrap();
    drop(wtr);

    let content = String::from_utf8(content).unwrap();
    snapbox::assert_eq_path(asset_path, content);
}

fn parse_dict(path: &str) -> Vec<(String, Vec<String>)> {
    let data = std::fs::read(path).unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(&*data);

    reader
        .records()
        .into_iter()
        .map(Result::unwrap)
        .map(|record| {
            let mut iter = record.into_iter();
            let typo = iter.next().expect("typo");
            (
                typo.to_owned(),
                iter.map(ToOwned::to_owned).collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn dict_from_iter<S: Into<String>>(
    iter: impl IntoIterator<Item = (S, impl IntoIterator<Item = S>)>,
) -> Dict {
    let mut dict = Dict::new();

    for (typo, corrections) in iter {
        let typo = UniCase::new(typo.into().to_ascii_lowercase());

        // duplicate entries are merged
        dict.entry(typo)
            .or_default()
            .extend(corrections.into_iter().map(|c| {
                let mut c = c.into();
                c.make_ascii_lowercase();
                c
            }));
    }

    dict
}

fn process<S: Into<String>>(
    iter: impl IntoIterator<Item = (S, impl IntoIterator<Item = S>)>,
) -> Dict {
    let dict = dict_from_iter(iter);

    let rows: Dict = dict
        .into_iter()
        .filter(|(t, _)| is_word(t))
        .map(|(t, c)| {
            let new_c: IndexSet<_> = c.into_iter().filter(|c| is_word(c)).collect();
            (t, new_c)
        })
        .collect();

    let varcon_words = varcon_words();
    let allowed_words = allowed_words();
    let word_variants = proper_word_variants();
    let top_1000_most_frequent_english_words = top_1000_most_frequent_english_words();
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|(typo, _)| {
            let is_disallowed = varcon_words.contains(&unicase::UniCase::new(typo));
            if is_disallowed {
                eprintln!("{:?} is disallowed; in varcon", typo);
            }
            !is_disallowed
        })
        .filter(|(typo, _)| {
            if let Some(reason) = allowed_words.get(typo.as_ref()) {
                eprintln!("{:?} is disallowed; {}", typo, reason);
                false
            } else {
                true
            }
        })
        .map(|(typo, corrections)| {
            let mut new_corrections = Vec::new();
            for correction in corrections {
                let correction = word_variants
                    .get(correction.as_str())
                    .and_then(|words| find_best_match(&typo, correction.as_str(), words))
                    .unwrap_or(&correction);
                new_corrections.push(correction.to_owned());
            }
            if let [only_correction] = &new_corrections[..] {
                if correction_should_contain_space(
                    &typo,
                    only_correction,
                    &top_1000_most_frequent_english_words,
                ) {
                    // We cannot provide corrections since we don't yet support corrections
                    // containing spaces (see https://github.com/crate-ci/typos/issues/795),
                    // so we just clear the corrections (which still disallows the typo).
                    new_corrections.clear();
                }
            }
            (typo, new_corrections.into_iter().collect::<IndexSet<_>>())
        })
        .collect();
    let mut dict = Dict::new();
    for (bad, good) in rows {
        let current = dict.entry(bad).or_default();
        current.extend(good);
    }

    let corrections: HashMap<_, _> = dict
        .iter()
        .flat_map(|(bad, good)| good.iter().map(|good| (good.to_owned(), bad.to_owned())))
        .collect();
    dict.into_iter()
        .filter(|(typo, _)| {
            if let Some(correction) = corrections.get(typo.as_str()) {
                eprintln!("{typo} <-> {correction} cycle detected");
                false
            } else {
                true
            }
        })
        .collect()
}

#[test]
fn test_preserve_correction_order() {
    let dict = process([("foo", ["xyz", "abc"])]);
    let mut corrections = dict.get(&UniCase::new("foo".into())).unwrap().iter();
    assert_eq!(corrections.next().unwrap(), "xyz");
    assert_eq!(corrections.next().unwrap(), "abc");
}

#[test]
fn test_merge_duplicates() {
    assert_eq!(
        process([("foo", ["bar"]), ("foo", ["baz"])]),
        dict_from_iter([("foo", ["bar", "baz"])])
    );
}

#[test]
fn test_duplicate_correction_removal() {
    let dict = process([("foo", ["bar", "bar"])]);
    assert_eq!(dict, dict_from_iter([("foo", ["bar"])]));
}

#[test]
fn test_cycle_removal() {
    assert!(process([("foo", ["bar"]), ("bar", ["foo"])]).is_empty());
}

#[test]
fn test_varcon_removal() {
    assert!(process([("colour", ["color"])]).is_empty());
}

#[test]
fn test_varcon_best_match() {
    assert_eq!(
        process([(
            "neighourhood", // note the missing 'b'
            ["neighborhood"],
        )]),
        dict_from_iter([(
            "neighourhood",
            ["neighbourhood"] // note that 'bor' has become 'bour' to match the typo
        )])
    );
}

#[test]
fn test_remove_only_correction_that_should_contain_space() {
    assert_eq!(
        process([("includea", ["include"],)]),
        dict_from_iter([("includea", [],)])
    );

    assert_eq!(
        process([("ainclude", ["include"],)]),
        dict_from_iter([("ainclude", [],)])
    );

    assert_eq!(
        process([("extrememe", ["extreme"],)]),
        dict_from_iter([("extrememe", ["extreme"],)])
    );

    assert_eq!(
        process([("mememory", ["memory"],)]),
        dict_from_iter([("mememory", ["memory"],)])
    );
}

fn is_word(word: &str) -> bool {
    word.chars().all(|c| c.is_alphabetic())
}

fn varcon_words() -> HashSet<unicase::UniCase<&'static str>> {
    // Even include improper ones because we should be letting varcon handle that rather than our
    // dictionary
    varcon::VARCON
        .iter()
        .flat_map(|c| c.entries.iter())
        .flat_map(|e| e.variants.iter())
        .map(|v| unicase::UniCase::new(v.word))
        .collect()
}

fn proper_word_variants() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut words: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
    for entry in varcon::VARCON.iter().flat_map(|c| c.entries.iter()) {
        let variants: HashSet<_> = entry
            .variants
            .iter()
            .filter(|v| v.types.iter().any(|t| t.tag != Some(varcon::Tag::Improper)))
            .map(|v| v.word)
            .collect();
        for variant in variants.iter() {
            let set = words.entry(variant).or_insert_with(HashSet::new);
            set.extend(variants.iter().filter(|v| *v != variant));
        }
    }
    words
}

fn find_best_match<'c>(
    typo: &'c str,
    correction: &'c str,
    word_variants: &HashSet<&'static str>,
) -> Option<&'c str> {
    assert!(!word_variants.contains(correction));
    let current = edit_distance::edit_distance(typo, correction);
    let mut matches: Vec<_> = word_variants
        .iter()
        .map(|r| (edit_distance::edit_distance(typo, r), *r))
        .filter(|(d, _)| *d < current)
        .collect();
    matches.sort_unstable();
    matches.into_iter().next().map(|(_, r)| r)
}

fn allowed_words() -> std::collections::HashMap<String, String> {
    let allowed_path = "assets/allowed.csv";
    let data = std::fs::read(allowed_path).unwrap();
    csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(data.as_slice())
        .records()
        .map(Result::unwrap)
        .map(|r| {
            let mut i = r.iter();
            let mut typo = i.next().expect("typo").to_owned();
            typo.make_ascii_lowercase();
            let reason = i.next().expect("reason").to_owned();
            (typo, reason)
        })
        .collect()
}

fn top_1000_most_frequent_english_words() -> HashSet<String> {
    std::fs::read_to_string("assets/top-1000-most-frequent-words.csv")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

/// Returns true if the typo equals the only correction immediately preceded or
/// followed by a common English word.  (Unless the only correction also
/// starts/ends in the common English word.)
///
/// The reasoning behind this is that we don't want to correct e.g. `includea`
/// to `include` since it might just be missing a space (`include a`).
/// On the other hand we still want to correct e.g. `extrememe` to `extreme`.
fn correction_should_contain_space(
    typo: &str,
    only_correction: &str,
    most_frequent_english_words: &HashSet<String>,
) -> bool {
    if let Some(prefix) = typo.strip_suffix(only_correction) {
        if most_frequent_english_words.contains(prefix) {
            if only_correction.starts_with(prefix) {
                return false;
            }

            return true;
        }
    }

    if let Some(suffix) = typo.strip_prefix(only_correction) {
        if most_frequent_english_words.contains(suffix) {
            if only_correction.ends_with(suffix) {
                return false;
            }

            return true;
        }
    }

    false
}
