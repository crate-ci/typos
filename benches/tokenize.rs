#![feature(test)]

extern crate test;

mod data;

#[bench]
fn symbol_parse_empty(b: &mut test::Bencher) {
    b.iter(|| typos::tokens::Identifier::parse_bytes(data::EMPTY.as_bytes()).last());
}

#[bench]
fn symbol_parse_no_tokens(b: &mut test::Bencher) {
    b.iter(|| typos::tokens::Identifier::parse_bytes(data::NO_TOKENS.as_bytes()).last());
}

#[bench]
fn symbol_parse_single_token(b: &mut test::Bencher) {
    b.iter(|| {
        typos::tokens::Identifier::parse_bytes(data::SINGLE_TOKEN.as_bytes()).last();
    });
}

#[bench]
fn symbol_parse_sherlock(b: &mut test::Bencher) {
    b.iter(|| typos::tokens::Identifier::parse_bytes(data::SHERLOCK.as_bytes()).last());
}

#[bench]
fn symbol_parse_code(b: &mut test::Bencher) {
    b.iter(|| typos::tokens::Identifier::parse_bytes(data::CODE.as_bytes()).last());
}

#[bench]
fn symbol_parse_corpus(b: &mut test::Bencher) {
    b.iter(|| typos::tokens::Identifier::parse_bytes(data::CORPUS.as_bytes()).last());
}

#[bench]
fn symbol_split_lowercase_short(b: &mut test::Bencher) {
    let input = "abcabcabcabc";
    let symbol = typos::tokens::Identifier::new(input, 0).unwrap();
    b.iter(|| symbol.split().last());
}

#[bench]
fn symbol_split_lowercase_long(b: &mut test::Bencher) {
    let input = "abcabcabcabc".repeat(90);
    let symbol = typos::tokens::Identifier::new(&input, 0).unwrap();
    b.iter(|| symbol.split().last());
}

#[bench]
fn symbol_split_mixed_short(b: &mut test::Bencher) {
    let input = "abcABCAbc123";
    let symbol = typos::tokens::Identifier::new(input, 0).unwrap();
    b.iter(|| symbol.split().last());
}

#[bench]
fn symbol_split_mixed_long(b: &mut test::Bencher) {
    let input = "abcABCAbc123".repeat(90);
    let symbol = typos::tokens::Identifier::new(&input, 0).unwrap();
    b.iter(|| symbol.split().last());
}
