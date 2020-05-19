use structopt::StructOpt;

pub const DICT: &str = include_str!("../../assets/dictionary.txt");

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

    let dict = parse_dict(DICT);

    writeln!(
        file,
        "pub static WORD_DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &[&'static str]> = ",
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for (typo, corrections) in dict {
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
