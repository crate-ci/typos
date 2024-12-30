pub const DICT: &str = include_str!("../assets/dictionary.txt");

#[test]
fn codegen() {
    let mut content = vec![];
    generate(&mut content);

    let content = String::from_utf8(content).unwrap();
    let content = codegenrs::rustfmt(&content, None).unwrap();
    snapbox::assert_data_eq!(content, snapbox::file!["../src/dict_codegen.rs"].raw());
}

#[test]
fn compat() {
    use std::fmt::Write as _;

    let mut content = String::new();
    for (bad, good) in parse_dict(DICT) {
        if !is_word(bad) {
            continue;
        }
        if !good.iter().copied().all(is_word) {
            continue;
        }
        let bad = bad.to_lowercase();
        write!(content, "{bad}").unwrap();
        for good in good {
            let good = good.to_lowercase();
            write!(content, ",{good}").unwrap();
        }
        writeln!(content).unwrap();
    }

    snapbox::assert_data_eq!(content, snapbox::file!["../assets/compatible.csv"].raw());
}

fn is_word(word: &str) -> bool {
    let tokenizer = typos::tokens::Tokenizer::new();

    tokenizer.parse_str(word).flat_map(|t| t.split()).count() == 1 && !word.contains('_')
}

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is @generated {}",
        file!().replace('\\', "/")
    )
    .unwrap();
    writeln!(file).unwrap();

    let dict = parse_dict(DICT);

    dictgen::DictGen::new()
        .name("WORD_DICTIONARY")
        .value_type("&[&str]")
        .ordered_map()
        .write(file, dict.map(|kv| (kv.0, format!("&{:?}", kv.1))))
        .unwrap();
}

fn parse_dict(raw: &str) -> impl Iterator<Item = (&str, Vec<&str>)> {
    raw.lines().map(|s| {
        let mut parts = s.splitn(2, "->");
        let typo = parts.next().unwrap().trim();
        let corrections = parts
            .next()
            .unwrap()
            .split(',')
            .filter_map(|c| {
                let c = c.trim();
                if c.is_empty() {
                    None
                } else {
                    Some(c)
                }
            })
            .collect();
        (typo, corrections)
    })
}
