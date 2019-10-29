use std::collections::HashMap;

use structopt::StructOpt;

pub const DICT: &str = include_str!("../../assets/words.go");

fn parse_dict(
    raw: &str,
) -> (
    HashMap<&str, Vec<&str>>,
    HashMap<&str, Vec<&str>>,
    HashMap<&str, Vec<&str>>,
) {
    let mut bad = HashMap::new();
    let mut main = HashMap::new();
    let mut american = HashMap::new();
    let mut british = HashMap::new();

    let mapping = regex::Regex::new(r#"^"(.*)", "(.*)",$"#).unwrap();

    let mut current = &mut bad;
    for line in raw.lines() {
        let line = line.splitn(2, "//").next().unwrap().trim();
        if line.is_empty() {
            continue;
        } else if line.starts_with("package") {
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
    (main, american, british)
}

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file, "#![allow(clippy::unreadable_literal)]",).unwrap();
    writeln!(file).unwrap();
    writeln!(file, "use unicase::UniCase;").unwrap();

    let (main, american, british) = parse_dict(DICT);

    writeln!(
        file,
        "pub static MAIN_DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &[&'static str]> = ",
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for (typo, corrections) in main {
        let value = itertools::join(corrections.iter().map(|s| format!("{:?}", s)), ", ");
        let value = format!("&[{}]", value);
        builder.entry(unicase::UniCase::new(typo), &value);
    }
    let codegenned = builder.build();
    writeln!(file, "{}", codegenned).unwrap();
    writeln!(file, ";").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "pub static AMERICAN_DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &[&'static str]> = ",
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for (typo, corrections) in american {
        let value = itertools::join(corrections.iter().map(|s| format!("{:?}", s)), ", ");
        let value = format!("&[{}]", value);
        builder.entry(unicase::UniCase::new(typo), &value);
    }
    let codegenned = builder.build();
    writeln!(file, "{}", codegenned).unwrap();
    writeln!(file, ";").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "pub static BRITISH_DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &[&'static str]> = ",
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for (typo, corrections) in british {
        let value = itertools::join(corrections.iter().map(|s| format!("{:?}", s)), ", ");
        let value = format!("&[{}]", value);
        builder.entry(unicase::UniCase::new(typo), &value);
    }
    let codegenned = builder.build();
    writeln!(file, "{}", codegenned).unwrap();
    writeln!(file, ";").unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(flatten)]
    codegen: codegenrs::CodeGenArgs,
    #[structopt(flatten)]
    rustmft: codegenrs::RustfmtArgs,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::from_args();

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
