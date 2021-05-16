use structopt::StructOpt;

const DICT: &[u8] = include_bytes!("../../assets/words.csv");

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

    let mut smallest = usize::MAX;
    let mut largest = usize::MIN;

    writeln!(
        file,
        "pub static WORD_DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &'static [&'static str]> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    let records: Vec<_> = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(DICT)
        .records()
        .map(|r| r.unwrap())
        .collect();
    for record in &records {
        let mut record_fields = record.iter();
        let key = record_fields.next().unwrap();
        smallest = std::cmp::min(smallest, key.len());
        largest = std::cmp::max(largest, key.len());
        let value = format!(
            "&[{}]",
            itertools::join(record_fields.map(|field| format!(r#""{}""#, field)), ", ")
        );
        builder.entry(unicase::UniCase::new(&record[0]), &value);
    }
    let codegenned = builder.build();
    writeln!(file, "{}", codegenned).unwrap();
    writeln!(file, ";").unwrap();
    writeln!(file).unwrap();
    writeln!(
        file,
        "pub const WORD_RANGE: std::ops::RangeInclusive<usize> = {}..={};",
        smallest, largest
    )
    .unwrap();
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
