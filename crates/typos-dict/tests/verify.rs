use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use unicase::UniCase;

type Dict = BTreeMap<UniCase<String>, Vec<String>>;

#[test]
fn verify() {
    let asset_path = "assets/words.csv";
    let data = std::fs::read(asset_path).unwrap();

    let mut content = vec![];
    generate(&mut content, &data);

    let content = String::from_utf8(content).unwrap();
    snapbox::assert_eq_path(asset_path, content);
}

fn generate<W: std::io::Write>(file: &mut W, dict: &[u8]) {
    let mut rows = Dict::new();
    csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(dict)
        .records()
        .map(Result::unwrap)
        .for_each(|r| {
            let mut i = r.iter();
            let mut typo = i.next().expect("typo").to_owned();
            typo.make_ascii_lowercase();
            let typo = UniCase::new(typo);
            rows.entry(typo).or_insert_with(Vec::new).extend(i.map(|c| {
                let mut c = c.to_owned();
                c.make_ascii_lowercase();
                c
            }));
        });

    let rows: Dict = rows
        .into_iter()
        .filter(|(t, _)| is_word(t))
        .filter_map(|(t, c)| {
            let new_c: Vec<_> = c.into_iter().filter(|c| is_word(c)).collect();
            if new_c.is_empty() {
                None
            } else {
                Some((t, new_c))
            }
        })
        .collect();

    let varcon_words = varcon_words();
    let allowed_words = allowed_words();
    let word_variants = proper_word_variants();
    let rows: Dict = rows
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
            let mut new_corrections = vec![];
            for correction in corrections {
                let correction = word_variants
                    .get(correction.as_str())
                    .and_then(|words| find_best_match(&typo, correction.as_str(), words))
                    .unwrap_or(&correction);
                new_corrections.push(correction.to_owned());
            }
            (typo, new_corrections)
        })
        .collect();

    let corrections: std::collections::HashSet<_> =
        rows.values().flatten().map(ToOwned::to_owned).collect();
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|(typo, _)| !corrections.contains(typo.as_str()))
        .collect();

    let mut wtr = csv::WriterBuilder::new().flexible(true).from_writer(file);
    for (typo, corrections) in rows {
        let mut row = corrections;
        row.insert(0, typo.as_str().to_owned());
        wtr.write_record(&row).unwrap();
    }
    wtr.flush().unwrap();
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
