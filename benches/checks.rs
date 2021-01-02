#![feature(test)]

extern crate test;

mod data;

use assert_fs::prelude::*;
use typos_cli::checks::FileChecker;

fn bench_files(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_files();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            &corrections,
            &typos_cli::report::PrintSilent,
        )
    });

    temp.close().unwrap();
}

#[bench]
fn files_empty(b: &mut test::Bencher) {
    bench_files(data::EMPTY, b);
}

#[bench]
fn files_no_tokens(b: &mut test::Bencher) {
    bench_files(data::NO_TOKENS, b);
}

#[bench]
fn files_single_token(b: &mut test::Bencher) {
    bench_files(data::SINGLE_TOKEN, b);
}

#[bench]
fn files_sherlock(b: &mut test::Bencher) {
    bench_files(data::SHERLOCK, b);
}

#[bench]
fn files_code(b: &mut test::Bencher) {
    bench_files(data::CODE, b);
}

#[bench]
fn files_corpus(b: &mut test::Bencher) {
    bench_files(data::CORPUS, b);
}

fn bench_identifiers(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_identifier_parser();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            &corrections,
            &typos_cli::report::PrintSilent,
        )
    });

    temp.close().unwrap();
}

#[bench]
fn identifiers_empty(b: &mut test::Bencher) {
    bench_identifiers(data::EMPTY, b);
}

#[bench]
fn identifiers_no_tokens(b: &mut test::Bencher) {
    bench_identifiers(data::NO_TOKENS, b);
}

#[bench]
fn identifiers_single_token(b: &mut test::Bencher) {
    bench_identifiers(data::SINGLE_TOKEN, b);
}

#[bench]
fn identifiers_sherlock(b: &mut test::Bencher) {
    bench_identifiers(data::SHERLOCK, b);
}

#[bench]
fn identifiers_code(b: &mut test::Bencher) {
    bench_identifiers(data::CODE, b);
}

#[bench]
fn identifiers_corpus(b: &mut test::Bencher) {
    bench_identifiers(data::CORPUS, b);
}

fn bench_words(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_word_parser();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            &corrections,
            &typos_cli::report::PrintSilent,
        )
    });

    temp.close().unwrap();
}

#[bench]
fn words_empty(b: &mut test::Bencher) {
    bench_words(data::EMPTY, b);
}

#[bench]
fn words_no_tokens(b: &mut test::Bencher) {
    bench_words(data::NO_TOKENS, b);
}

#[bench]
fn words_single_token(b: &mut test::Bencher) {
    bench_words(data::SINGLE_TOKEN, b);
}

#[bench]
fn words_sherlock(b: &mut test::Bencher) {
    bench_words(data::SHERLOCK, b);
}

#[bench]
fn words_code(b: &mut test::Bencher) {
    bench_words(data::CODE, b);
}

#[bench]
fn words_corpus(b: &mut test::Bencher) {
    bench_words(data::CORPUS, b);
}

fn bench_typos(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_typos();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            &corrections,
            &typos_cli::report::PrintSilent,
        )
    });

    temp.close().unwrap();
}

#[bench]
fn typos_empty(b: &mut test::Bencher) {
    bench_typos(data::EMPTY, b);
}

#[bench]
fn typos_no_tokens(b: &mut test::Bencher) {
    bench_typos(data::NO_TOKENS, b);
}

#[bench]
fn typos_single_token(b: &mut test::Bencher) {
    bench_typos(data::SINGLE_TOKEN, b);
}

#[bench]
fn typos_sherlock(b: &mut test::Bencher) {
    bench_typos(data::SHERLOCK, b);
}

#[bench]
fn typos_code(b: &mut test::Bencher) {
    bench_typos(data::CODE, b);
}

#[bench]
fn typos_corpus(b: &mut test::Bencher) {
    bench_typos(data::CORPUS, b);
}
