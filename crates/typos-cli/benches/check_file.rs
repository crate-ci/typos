#![allow(elided_lifetimes_in_paths)]

mod data;

use assert_fs::prelude::*;
use typos_cli::file::FileChecker;

#[divan::bench(args = data::DATA)]
fn found_files(bencher: divan::Bencher, sample: &data::Data) {
    let dict = typos_cli::dict::BuiltIn::new(Default::default());
    let tokenizer = typos::tokens::Tokenizer::new();
    let policy = typos_cli::policy::Policy::new()
        .dict(&dict)
        .tokenizer(&tokenizer);

    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child(sample.name());
    sample_path.write_str(sample.content()).unwrap();

    bencher
        .counter(divan::counter::BytesCount::of_str(sample.content()))
        .bench_local(|| {
            typos_cli::file::FoundFiles.check_file(sample_path.path(), true, &policy, &PrintSilent)
        });
}

#[divan::bench(args = data::DATA)]
fn identifiers(bencher: divan::Bencher, sample: &data::Data) {
    let dict = typos_cli::dict::BuiltIn::new(Default::default());
    let tokenizer = typos::tokens::Tokenizer::new();
    let policy = typos_cli::policy::Policy::new()
        .dict(&dict)
        .tokenizer(&tokenizer);

    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child(sample.name());
    sample_path.write_str(sample.content()).unwrap();

    bencher
        .counter(divan::counter::BytesCount::of_str(sample.content()))
        .bench_local(|| {
            typos_cli::file::Identifiers.check_file(sample_path.path(), true, &policy, &PrintSilent)
        });
}

#[divan::bench(args = data::DATA)]
fn words(bencher: divan::Bencher, sample: &data::Data) {
    let dict = typos_cli::dict::BuiltIn::new(Default::default());
    let tokenizer = typos::tokens::Tokenizer::new();
    let policy = typos_cli::policy::Policy::new()
        .dict(&dict)
        .tokenizer(&tokenizer);

    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child(sample.name());
    sample_path.write_str(sample.content()).unwrap();

    bencher
        .counter(divan::counter::BytesCount::of_str(sample.content()))
        .bench_local(|| {
            typos_cli::file::Words.check_file(sample_path.path(), true, &policy, &PrintSilent)
        });
}

#[divan::bench(args = data::DATA)]
fn typos(bencher: divan::Bencher, sample: &data::Data) {
    let dict = typos_cli::dict::BuiltIn::new(Default::default());
    let tokenizer = typos::tokens::Tokenizer::new();
    let policy = typos_cli::policy::Policy::new()
        .dict(&dict)
        .tokenizer(&tokenizer);

    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child(sample.name());
    sample_path.write_str(sample.content()).unwrap();

    bencher
        .counter(divan::counter::BytesCount::of_str(sample.content()))
        .bench_local(|| {
            typos_cli::file::Typos.check_file(sample_path.path(), true, &policy, &PrintSilent)
        });
}

#[derive(Debug, Default)]
pub struct PrintSilent;

impl typos_cli::report::Report for PrintSilent {
    fn report(&self, _msg: typos_cli::report::Message) -> Result<(), std::io::Error> {
        Ok(())
    }
}

fn main() {
    divan::main();
}
