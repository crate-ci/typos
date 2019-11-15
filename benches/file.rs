#![feature(test)]

extern crate test;

mod data;

use assert_fs::prelude::*;
use bstr::ByteSlice;

fn bench_read(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn read_empty(b: &mut test::Bencher) {
    bench_read(data::EMPTY, b);
}

#[bench]
fn read_no_tokens(b: &mut test::Bencher) {
    bench_read(data::NO_TOKENS, b);
}

#[bench]
fn read_single_token(b: &mut test::Bencher) {
    bench_read(data::SINGLE_TOKEN, b);
}

#[bench]
fn read_sherlock(b: &mut test::Bencher) {
    bench_read(data::SHERLOCK, b);
}

#[bench]
fn read_code(b: &mut test::Bencher) {
    bench_read(data::CODE, b);
}

#[bench]
fn read_corpus(b: &mut test::Bencher) {
    bench_read(data::CORPUS, b);
}

fn bench_split_lines(data: &str, b: &mut test::Bencher) {
    b.iter(|| data.as_bytes().lines().enumerate().last());
}

#[bench]
fn parse_lines_empty(b: &mut test::Bencher) {
    bench_split_lines(data::EMPTY, b);
}

#[bench]
fn parse_lines_no_tokens(b: &mut test::Bencher) {
    bench_split_lines(data::NO_TOKENS, b);
}

#[bench]
fn parse_lines_single_token(b: &mut test::Bencher) {
    bench_split_lines(data::SINGLE_TOKEN, b);
}

#[bench]
fn parse_lines_sherlock(b: &mut test::Bencher) {
    bench_split_lines(data::SHERLOCK, b);
}

#[bench]
fn parse_lines_code(b: &mut test::Bencher) {
    bench_split_lines(data::CODE, b);
}

#[bench]
fn parse_lines_corpus(b: &mut test::Bencher) {
    bench_split_lines(data::CORPUS, b);
}

fn bench_parse_ident(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::TyposSettings::new().build_identifier_parser();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            typos::report::print_silent,
        )
    });

    temp.close().unwrap();
}

#[bench]
fn parse_idents_empty(b: &mut test::Bencher) {
    bench_parse_ident(data::EMPTY, b);
}

#[bench]
fn parse_idents_no_tokens(b: &mut test::Bencher) {
    bench_parse_ident(data::NO_TOKENS, b);
}

#[bench]
fn parse_idents_single_token(b: &mut test::Bencher) {
    bench_parse_ident(data::SINGLE_TOKEN, b);
}

#[bench]
fn parse_idents_sherlock(b: &mut test::Bencher) {
    bench_parse_ident(data::SHERLOCK, b);
}

#[bench]
fn parse_idents_code(b: &mut test::Bencher) {
    bench_parse_ident(data::CODE, b);
}

#[bench]
fn parse_idents_corpus(b: &mut test::Bencher) {
    bench_parse_ident(data::CORPUS, b);
}

fn bench_parse_word(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::TyposSettings::new().build_word_parser();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            typos::report::print_silent,
        )
    });

    temp.close().unwrap();
}

#[bench]
fn parse_words_empty(b: &mut test::Bencher) {
    bench_parse_word(data::EMPTY, b);
}

#[bench]
fn parse_words_no_tokens(b: &mut test::Bencher) {
    bench_parse_word(data::NO_TOKENS, b);
}

#[bench]
fn parse_words_single_token(b: &mut test::Bencher) {
    bench_parse_word(data::SINGLE_TOKEN, b);
}

#[bench]
fn parse_words_sherlock(b: &mut test::Bencher) {
    bench_parse_word(data::SHERLOCK, b);
}

#[bench]
fn parse_words_code(b: &mut test::Bencher) {
    bench_parse_word(data::CODE, b);
}

#[bench]
fn parse_words_corpus(b: &mut test::Bencher) {
    bench_parse_word(data::CORPUS, b);
}

fn bench_check_file(data: &str, b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data).unwrap();

    let corrections = typos_cli::dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::TyposSettings::new().build_checks();
    b.iter(|| {
        checks.check_file(
            sample_path.path(),
            true,
            &parser,
            &corrections,
            typos::report::print_silent,
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
