#![feature(test)]

extern crate test;

mod data;

use assert_fs::prelude::*;
use typos_cli::checks::Check;

fn bench_parse_ident_str(data: &str, b: &mut test::Bencher) {
    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_identifier_parser();
    b.iter(|| checks.check_str(data, &parser, &corrections, &typos_cli::report::PrintSilent));
}

#[bench]
fn parse_idents_empty_str(b: &mut test::Bencher) {
    bench_parse_ident_str(data::EMPTY, b);
}

#[bench]
fn parse_idents_no_tokens_str(b: &mut test::Bencher) {
    bench_parse_ident_str(data::NO_TOKENS, b);
}

#[bench]
fn parse_idents_single_token_str(b: &mut test::Bencher) {
    bench_parse_ident_str(data::SINGLE_TOKEN, b);
}

#[bench]
fn parse_idents_sherlock_str(b: &mut test::Bencher) {
    bench_parse_ident_str(data::SHERLOCK, b);
}

#[bench]
fn parse_idents_code_str(b: &mut test::Bencher) {
    bench_parse_ident_str(data::CODE, b);
}

#[bench]
fn parse_idents_corpus_str(b: &mut test::Bencher) {
    bench_parse_ident_str(data::CORPUS, b);
}

fn bench_parse_ident_bytes(data: &str, b: &mut test::Bencher) {
    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_identifier_parser();
    b.iter(|| {
        checks.check_bytes(
            data.as_bytes(),
            &parser,
            &corrections,
            &typos_cli::report::PrintSilent,
        )
    });
}

#[bench]
fn parse_idents_empty_bytes(b: &mut test::Bencher) {
    bench_parse_ident_bytes(data::EMPTY, b);
}

#[bench]
fn parse_idents_no_tokens_bytes(b: &mut test::Bencher) {
    bench_parse_ident_bytes(data::NO_TOKENS, b);
}

#[bench]
fn parse_idents_single_token_bytes(b: &mut test::Bencher) {
    bench_parse_ident_bytes(data::SINGLE_TOKEN, b);
}

#[bench]
fn parse_idents_sherlock_bytes(b: &mut test::Bencher) {
    bench_parse_ident_bytes(data::SHERLOCK, b);
}

#[bench]
fn parse_idents_code_bytes(b: &mut test::Bencher) {
    bench_parse_ident_bytes(data::CODE, b);
}

#[bench]
fn parse_idents_corpus_bytes(b: &mut test::Bencher) {
    bench_parse_ident_bytes(data::CORPUS, b);
}

fn bench_parse_word_str(data: &str, b: &mut test::Bencher) {
    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_word_parser();
    b.iter(|| checks.check_str(data, &parser, &corrections, &typos_cli::report::PrintSilent));
}

#[bench]
fn parse_words_empty(b: &mut test::Bencher) {
    bench_parse_word_str(data::EMPTY, b);
}

#[bench]
fn parse_words_no_tokens(b: &mut test::Bencher) {
    bench_parse_word_str(data::NO_TOKENS, b);
}

#[bench]
fn parse_words_single_token(b: &mut test::Bencher) {
    bench_parse_word_str(data::SINGLE_TOKEN, b);
}

#[bench]
fn parse_words_sherlock(b: &mut test::Bencher) {
    bench_parse_word_str(data::SHERLOCK, b);
}

#[bench]
fn parse_words_code(b: &mut test::Bencher) {
    bench_parse_word_str(data::CODE, b);
}

#[bench]
fn parse_words_corpus(b: &mut test::Bencher) {
    bench_parse_word_str(data::CORPUS, b);
}

fn bench_typos(data: &str, b: &mut test::Bencher) {
    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_typos();
    b.iter(|| checks.check_str(data, &parser, &corrections, &typos_cli::report::PrintSilent));
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

fn bench_check_file(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let parser = typos::tokens::Tokenizer::new();
    let checks = typos_cli::checks::TyposSettings::new().build_typos();
    b.iter(|| {
        checks.check_file_content(
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
fn check_file_empty(b: &mut test::Bencher) {
    bench_check_file(data::EMPTY, b);
}

#[bench]
fn check_file_no_tokens(b: &mut test::Bencher) {
    bench_check_file(data::NO_TOKENS, b);
}

#[bench]
fn check_file_single_token(b: &mut test::Bencher) {
    bench_check_file(data::SINGLE_TOKEN, b);
}

#[bench]
fn check_file_sherlock(b: &mut test::Bencher) {
    bench_check_file(data::SHERLOCK, b);
}

#[bench]
fn check_file_code(b: &mut test::Bencher) {
    bench_check_file(data::CODE, b);
}

#[bench]
fn check_file_corpus(b: &mut test::Bencher) {
    bench_check_file(data::CORPUS, b);
}
