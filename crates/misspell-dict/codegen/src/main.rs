use std::collections::HashMap;

use clap::Parser;

pub const DICT: &str = include_str!("../../assets/words.go");

struct Words<'s> {
    main: HashMap<&'s str, Vec<&'s str>>,
    american: HashMap<&'s str, Vec<&'s str>>,
    british: HashMap<&'s str, Vec<&'s str>>,
}

fn parse_dict(raw: &str) -> Words {
    let mut bad = HashMap::new();
    let mut main = HashMap::new();
    let mut american = HashMap::new();
    let mut british = HashMap::new();

    let mapping = regex::Regex::new(r#"^"(.*)", "(.*)",$"#).unwrap();

    let mut current = &mut bad;
    for line in raw.lines() {
        let line = line.splitn(2, "//").next().unwrap().trim();
        if line.is_empty() || line.starts_with("package") {
            continue;
        } else if line.contains("DictMain") {
            current = &mut main;
        } else if line.contains("DictAmerican") {
            current = &mut american;
        } else if line.contains("DictBritish") {
            current = &mut british;
        } else if line.contains('}') {
            current = &mut bad;
        } else {
            let captures = mapping.captures(line);
            if let Some(captures) = captures {
                current.insert(
                    captures.get(1).unwrap().as_str(),
                    vec![captures.get(2).unwrap().as_str()],
                );
            } else {
                eprintln!("Unknown line: {}", line);
            }
        }
    }

    if !bad.is_empty() {
        panic!("Failed parsing; found extra words: {:#?}", bad);
    }

    Words {
        main,
        american,
        british,
    }
}

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file).unwrap();

    let Words {
        main,
        american,
        british,
    } = parse_dict(DICT);

    dictgen::generate_table(
        file,
        "MAIN_DICTIONARY",
        "&[&str]",
        main.into_iter().map(|kv| (kv.0, format!("&{:?}", kv.1))),
    )
    .unwrap();

    dictgen::generate_table(
        file,
        "AMERICAN_DICTIONARY",
        "&[&str]",
        american
            .into_iter()
            .map(|kv| (kv.0, format!("&{:?}", kv.1))),
    )
    .unwrap();

    dictgen::generate_table(
        file,
        "BRITISH_DICTIONARY",
        "&[&str]",
        british.into_iter().map(|kv| (kv.0, format!("&{:?}", kv.1))),
    )
    .unwrap();
}

#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    codegen: codegenrs::CodeGenArgs,
    #[clap(flatten)]
    rustmft: codegenrs::RustfmtArgs,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::parse();

    let mut content = vec![];
    generate(&mut content);

    let content = String::from_utf8(content)?;
    let content = options.rustmft.reformat(&content)?;
    options.codegen.write_str(&content)?;

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
