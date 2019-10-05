use structopt::StructOpt;

fn generate<W: std::io::Write>(input: &[u8], file: &mut W) {
    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file).unwrap();
    writeln!(file, "use unicase::UniCase;").unwrap();

    writeln!(
        file,
        "pub(crate) static WORD_DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &'static str> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    let records: Vec<_> = csv::Reader::from_reader(input)
        .records()
        .map(|r| r.unwrap())
        .collect();
    for record in &records {
        let value = format!(r#""{}""#, &record[1]);
        builder.entry(unicase::UniCase(&record[0]), &value);
    }
    builder.build(file).unwrap();
    writeln!(file, ";").unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(long, parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(flatten)]
    codegen: codegenrs::CodeGenArgs,
    #[structopt(flatten)]
    rustmft: codegenrs::RustfmtArgs,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let content = {
        let mut content = vec![];
        let input = std::fs::read(&options.input)?;
        generate(&input, &mut content);
        content
    };

    let content = String::from_utf8(content)?;
    let content = options.rustmft.reformat(&content)?;
    options.codegen.write_str(&content)?;

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
