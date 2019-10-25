#![feature(test)]

extern crate test;

mod data;

use assert_fs::prelude::*;
use bstr::ByteSlice;

#[bench]
fn check_file_empty(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::EMPTY).unwrap();

    let corrections = typos_dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::CheckSettings::new().build(&corrections, &parser);
    b.iter(|| checks.check_file(sample_path.path(), true, typos::report::print_silent));

    temp.close().unwrap();
}

#[bench]
fn check_file_no_tokens(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::NO_TOKENS).unwrap();

    let corrections = typos_dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::CheckSettings::new().build(&corrections, &parser);
    b.iter(|| checks.check_file(sample_path.path(), true, typos::report::print_silent));

    temp.close().unwrap();
}

#[bench]
fn check_file_single_token(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::SINGLE_TOKEN).unwrap();

    let corrections = typos_dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::CheckSettings::new().build(&corrections, &parser);
    b.iter(|| checks.check_file(sample_path.path(), true, typos::report::print_silent));

    temp.close().unwrap();
}

#[bench]
fn check_file_sherlock(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::SHERLOCK).unwrap();

    let corrections = typos_dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::CheckSettings::new().build(&corrections, &parser);
    b.iter(|| checks.check_file(sample_path.path(), true, typos::report::print_silent));

    temp.close().unwrap();
}

#[bench]
fn check_file_code(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::CODE).unwrap();

    let corrections = typos_dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::CheckSettings::new().build(&corrections, &parser);
    b.iter(|| checks.check_file(sample_path.path(), true, typos::report::print_silent));

    temp.close().unwrap();
}

#[bench]
fn check_file_corpus(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::CORPUS).unwrap();

    let corrections = typos_dict::BuiltIn::new();
    let parser = typos::tokens::Parser::new();
    let checks = typos::checks::CheckSettings::new().build(&corrections, &parser);
    b.iter(|| checks.check_file(sample_path.path(), true, typos::report::print_silent));

    temp.close().unwrap();
}

#[bench]
fn read_empty(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::EMPTY).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn read_no_tokens(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::NO_TOKENS).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn read_single_token(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::SINGLE_TOKEN).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn read_sherlock(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::SHERLOCK).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn read_code(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::CODE).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn read_corpus(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::CORPUS).unwrap();

    b.iter(|| std::fs::read(sample_path.path()));

    temp.close().unwrap();
}

#[bench]
fn split_lines_empty(b: &mut test::Bencher) {
    b.iter(|| data::EMPTY.as_bytes().lines().enumerate().last());
}

#[bench]
fn split_lines_no_tokens(b: &mut test::Bencher) {
    b.iter(|| data::NO_TOKENS.as_bytes().lines().enumerate().last());
}

#[bench]
fn split_lines_single_token(b: &mut test::Bencher) {
    b.iter(|| data::SINGLE_TOKEN.as_bytes().lines().enumerate().last());
}

#[bench]
fn split_lines_sherlock(b: &mut test::Bencher) {
    b.iter(|| data::SHERLOCK.as_bytes().lines().enumerate().last());
}

#[bench]
fn split_lines_code(b: &mut test::Bencher) {
    b.iter(|| data::CODE.as_bytes().lines().enumerate().last());
}

#[bench]
fn split_lines_corpus(b: &mut test::Bencher) {
    b.iter(|| data::CORPUS.as_bytes().lines().enumerate().last());
}

#[bench]
fn parse_empty(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::EMPTY
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).last();
                ()
            })
    });
}

#[bench]
fn parse_no_tokens(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::NO_TOKENS
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).last();
                ()
            })
    });
}

#[bench]
fn parse_single_token(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::SINGLE_TOKEN
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).last();
                ()
            })
    });
}

#[bench]
fn parse_sherlock(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::SHERLOCK
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).last();
                ()
            })
    });
}

#[bench]
fn parse_code(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::CODE
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).last();
                ()
            })
    });
}

#[bench]
fn parse_corpus(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::CORPUS
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).last();
                ()
            })
    });
}

#[bench]
fn split_empty(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::EMPTY
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).for_each(|l| {
                    l.split().last();
                    ()
                })
            })
    });
}

#[bench]
fn split_no_tokens(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::NO_TOKENS
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).for_each(|l| {
                    l.split().last();
                    ()
                })
            })
    });
}

#[bench]
fn split_single_token(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::SINGLE_TOKEN
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).for_each(|l| {
                    l.split().last();
                    ()
                })
            })
    });
}

#[bench]
fn split_sherlock(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::SHERLOCK
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).for_each(|l| {
                    l.split().last();
                    ()
                })
            })
    });
}

#[bench]
fn split_code(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::CODE
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).for_each(|l| {
                    l.split().last();
                    ()
                })
            })
    });
}

#[bench]
fn split_corpus(b: &mut test::Bencher) {
    let parser = typos::tokens::Parser::new();
    b.iter(|| {
        data::CORPUS
            .as_bytes()
            .lines()
            .enumerate()
            .for_each(|(_idx, l)| {
                parser.parse_bytes(l).for_each(|l| {
                    l.split().last();
                    ()
                })
            })
    });
}
