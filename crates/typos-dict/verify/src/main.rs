use std::collections::HashMap;
use std::collections::HashSet;

use structopt::StructOpt;

fn generate<W: std::io::Write>(file: &mut W, dict: &[u8]) {
    let mut wtr = csv::Writer::from_writer(file);

    let disallowed_typos = disallowed_typos();
    let related_words = related_words();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(dict);
    for record in reader.records() {
        let record = record.unwrap();
        let typo = &record[0];
        let correction = &record[1];
        if disallowed_typos.contains(&unicase::UniCase::new(typo)) {
            continue;
        }
        let correction = related_words
            .get(correction)
            .and_then(|words| find_best_match(typo, correction, words))
            .unwrap_or(correction);
        wtr.write_record(&[typo, correction]).unwrap();
    }
    wtr.flush().unwrap();
}

fn disallowed_typos() -> HashSet<unicase::UniCase<&'static str>> {
    varcon::VARCON
        .iter()
        .flat_map(|c| c.entries.iter())
        .flat_map(|e| e.variants.iter())
        .map(|v| unicase::UniCase::new(v.word))
        .collect()
}

fn related_words() -> HashMap<&'static str, HashSet<&'static str>> {
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
    related_words: &HashSet<&'static str>,
) -> Option<&'c str> {
    assert!(!related_words.contains(correction));
    let current = edit_distance::edit_distance(typo, correction);
    let mut matches: Vec<_> = related_words
        .iter()
        .map(|r| (edit_distance::edit_distance(typo, r), *r))
        .filter(|(d, _)| *d < current)
        .collect();
    matches.sort_unstable();
    matches.into_iter().next().map(|(_, r)| r)
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(short("-i"), long, parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(flatten)]
    codegen: codegenrs::CodeGenArgs,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let data = std::fs::read(&options.input).unwrap();

    let mut content = vec![];
    generate(&mut content, &data);

    let content = String::from_utf8(content)?;
    options.codegen.write_str(&content)?;

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
