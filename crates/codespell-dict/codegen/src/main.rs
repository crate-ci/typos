use clap::Parser;

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
    writeln!(file).unwrap();

    let dict = parse_dict(DICT);

    dictgen::generate_table(
        file,
        "WORD_DICTIONARY",
        "&[&str]",
        dict.map(|kv| (kv.0, format!("&{:?}", kv.1))),
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
