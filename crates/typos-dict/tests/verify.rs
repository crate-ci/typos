use indexmap::IndexSet;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use unicase::UniCase;

type Dict = BTreeMap<UniCase<String>, IndexSet<String>>;

#[test]
fn verify() {
    let typos_dict = parse_dict("assets/words.csv");
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
    snapbox::assert_data_eq!(content, snapbox::file!["../assets/words.csv"].raw());
}

fn parse_dict(path: &str) -> Vec<(String, Vec<String>)> {
    let data = std::fs::read(path).unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(&*data);

    reader
        .records()
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
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|(typo, _)| {
            let is_disallowed = varcon_words.contains(&UniCase::new(typo));
            if is_disallowed {
                eprintln!("{typo:?} is disallowed; in varcon");
            }
            !is_disallowed
        })
        .filter(|(typo, _)| {
            if let Some(reason) = allowed_words.get(typo.as_ref()) {
                eprintln!("{typo:?} is disallowed; {reason}");
                false
            } else {
                true
            }
        })
        .map(|(typo, corrections)| {
            let mut new_corrections = IndexSet::new();
            for correction in corrections {
                let correction = word_variants
                    .get(correction.as_str())
                    .and_then(|words| find_best_match(&typo, correction.as_str(), words))
                    .unwrap_or(&correction);
                new_corrections.insert(correction.to_owned());
            }
            (typo, new_corrections)
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
    assert!(process([("foo", ["foobar"]), ("foobar", ["foo"])]).is_empty());
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

fn is_word(word: &str) -> bool {
    word.chars().all(|c| c.is_alphabetic())
}

fn varcon_words() -> HashSet<UniCase<&'static str>> {
    // Even include improper ones because we should be letting varcon handle that rather than our
    // dictionary
    varcon::VARCON
        .iter()
        .filter(|c| c.verified)
        .flat_map(|c| c.entries.iter())
        .flat_map(|e| e.variants.iter())
        .map(|v| UniCase::new(v.word))
        .collect()
}

fn proper_word_variants() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut words: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
    for entry in varcon::VARCON
        .iter()
        .filter(|c| c.verified)
        .flat_map(|c| c.entries.iter())
    {
        let variants: HashSet<_> = entry
            .variants
            .iter()
            .filter(|v| v.types.iter().any(|t| t.tag != Some(varcon::Tag::Improper)))
            .map(|v| v.word)
            .collect();
        for variant in variants.iter() {
            let set = words.entry(variant).or_default();
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
    #[allow(clippy::single_match)]
    match (typo, correction) {
        // Picking the worst option due to a letter swap being an edit distance of two
        ("alinging", "aligning") | ("alingment", "alignment") | ("alingments", "alignments") => {
            return None;
        }
        _ => {}
    }
    let current = edit_distance::edit_distance(typo, correction);
    let mut matches: Vec<_> = word_variants
        .iter()
        .map(|r| (edit_distance::edit_distance(typo, r), *r))
        .filter(|(d, _)| *d < current)
        .collect();
    matches.sort_unstable();
    matches.into_iter().next().map(|(_, r)| r)
}

fn allowed_words() -> HashMap<String, String> {
    let allowed_path = "assets/english.csv";
    let english_data = std::fs::read(allowed_path).unwrap();
    let mut allowed_english = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(english_data.as_slice());
    let allowed_english = allowed_english.records().map(Result::unwrap).map(|r| {
        let mut i = r.iter();
        let mut typo = i.next().expect("typo").to_owned();
        typo.make_ascii_lowercase();
        (typo, String::from("english word"))
    });

    let allowed_path = "assets/allowed.csv";
    let local_data = std::fs::read(allowed_path).unwrap();
    let mut allowed_local = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(local_data.as_slice());
    let allowed_local = allowed_local.records().map(Result::unwrap).map(|r| {
        let mut i = r.iter();
        let mut typo = i.next().expect("typo").to_owned();
        typo.make_ascii_lowercase();
        let reason = i.next().expect("reason").to_owned();
        (typo, reason)
    });

    allowed_english.chain(allowed_local).collect()
}
