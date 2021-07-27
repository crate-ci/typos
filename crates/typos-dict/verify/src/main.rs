use std::collections::HashMap;
use std::collections::HashSet;

use structopt::StructOpt;

fn generate<W: std::io::Write>(file: &mut W, dict: &[u8]) {
    let rows: Vec<Vec<_>> = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(dict)
        .records()
        .map(Result::unwrap)
        .map(|r| {
            let row: Vec<String> = r.iter().map(ToOwned::to_owned).collect();
            row
        })
        .collect();

    let disallowed_typos = varcon_words();
    let word_variants = proper_word_variants();
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|r| {
            let typo = &r[0];
            let is_disallowed = disallowed_typos.contains(&unicase::UniCase::new(typo));
            if is_disallowed {
                eprintln!("{:?} is disallowed", typo);
            }
            !is_disallowed
        })
        .map(|r| {
            let mut fields = r.into_iter();
            let typo = fields.next().expect("at least a typo");
            let mut row = vec![typo.clone()];
            for correction in fields {
                let correction = word_variants
                    .get(correction.as_str())
                    .and_then(|words| find_best_match(&typo, correction.as_str(), words))
                    .unwrap_or(&correction);
                row.push(correction.to_owned());
            }
            row
        })
        .collect();

    let corrections: std::collections::HashSet<_> = rows
        .iter()
        .flat_map(|r| {
            let mut i = r.iter();
            i.next();
            i.map(ToOwned::to_owned)
        })
        .collect();
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|r| !corrections.contains(&r[0]))
        .collect();

    let mut wtr = csv::WriterBuilder::new().flexible(true).from_writer(file);
    for row in rows {
        wtr.write_record(&row).unwrap();
    }
    wtr.flush().unwrap();
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
