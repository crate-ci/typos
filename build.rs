use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub const CORPUS: &[u8] = include_bytes!("./assets/words.csv");

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    println!("rerun-if-changed=./assets/words.csv");
    write!(&mut file, "use unicase::UniCase;").unwrap();
    write!(
        &mut file,
        "static DICTIONARY: phf::Map<unicase::UniCase<&'static str>, &'static str> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    let records: Vec<_> = csv::Reader::from_reader(CORPUS)
        .records()
        .map(|r| r.unwrap())
        .collect();
    for record in &records {
        let value = format!(r#""{}""#, &record[1]);
        builder.entry(unicase::UniCase(&record[0]), &value);
    }
    builder.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
