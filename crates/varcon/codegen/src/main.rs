use structopt::StructOpt;

const DICT: &[u8] = include_bytes!("../../assets/varcon.txt");

fn generate<W: std::io::Write>(file: &mut W) {
    let dict = String::from_utf8_lossy(DICT);
    let clusters = varcon_core::ClusterIter::new(&dict);

    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file, "#![allow(clippy::unreadable_literal)]",).unwrap();
    writeln!(file).unwrap();
    writeln!(file, "use crate::*;").unwrap();
    writeln!(file).unwrap();

    writeln!(file, "pub static VARCON: &'static [Cluster] = &[").unwrap();
    for mut cluster in clusters {
        cluster.infer();
        writeln!(file, "Cluster {{").unwrap();
        writeln!(file, "  header: {:?},", cluster.header).unwrap();
        writeln!(file, "  entries: &[").unwrap();
        for entry in &cluster.entries {
            writeln!(file, "  Entry {{").unwrap();
            writeln!(file, "    variants: &[").unwrap();
            for variant in &entry.variants {
                writeln!(file, "      Variant {{").unwrap();
                writeln!(file, "        word: {:?},", variant.word).unwrap();
                writeln!(file, "        types: &[").unwrap();
                for t in &variant.types {
                    write!(file, "          Type {{").unwrap();
                    write!(file, "category: Category::{:?}, ", t.category).unwrap();
                    if let Some(tag) = t.tag {
                        write!(file, "tag: Some(Tag::{:?}), ", tag).unwrap();
                    } else {
                        write!(file, "tag: {:?}, ", t.tag).unwrap();
                    }
                    write!(file, "num: {:?},", t.num).unwrap();
                    writeln!(file, "}},").unwrap();
                }
                writeln!(file, "        ],").unwrap();
                writeln!(file, "      }},").unwrap();
            }
            writeln!(file, "  ],").unwrap();
            if let Some(pos) = entry.pos {
                write!(file, "  pos: Some(Pos::{:?}),", pos).unwrap();
            } else {
                write!(file, "  pos: {:?},", entry.pos).unwrap();
            }
            writeln!(
                file,
                " archaic: {:?}, note: {:?},",
                entry.archaic, entry.note
            )
            .unwrap();
            writeln!(file, "  description: {:?},", entry.description).unwrap();
            writeln!(file, "  comment: {:?},", entry.comment).unwrap();
            writeln!(file, "  }},").unwrap();
        }
        writeln!(file, "  ],").unwrap();
        writeln!(file, "  notes: &[").unwrap();
        for note in &cluster.notes {
            writeln!(file, "    {:?},", note).unwrap();
        }
        writeln!(file, "  ],").unwrap();
        writeln!(file, "  }},").unwrap();
    }
    writeln!(file, "];").unwrap();
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
