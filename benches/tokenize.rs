#![feature(test)]

extern crate test;

mod data;

#[bench]
fn symbol_parse_empty(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Identifier::parse(data::EMPTY.as_bytes()).last());
}

#[bench]
fn symbol_parse_no_tokens(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Identifier::parse(data::NO_TOKENS.as_bytes()).last());
}

#[bench]
fn symbol_parse_single_token(b: &mut test::Bencher) {
    b.iter(|| {
        defenestrate::tokens::Identifier::parse(data::SINGLE_TOKEN.as_bytes()).last();
    });
}

#[bench]
fn symbol_parse_sherlock(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Identifier::parse(data::SHERLOCK.as_bytes()).last());
}

#[bench]
fn symbol_parse_code(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Identifier::parse(data::CODE.as_bytes()).last());
}

#[bench]
fn symbol_parse_corpus(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Identifier::parse(data::CORPUS.as_bytes()).last());
}

#[bench]
fn symbol_split_lowercase_short(b: &mut test::Bencher) {
    let input = "abcabcabcabc";
    let symbol = defenestrate::tokens::Identifier::new(input, 0).unwrap();
    b.iter(|| symbol.split().last());
}

#[bench]
fn symbol_split_lowercase_long(b: &mut test::Bencher) {
    let input = "abcabcabcabc".repeat(90);
    let symbol = defenestrate::tokens::Identifier::new(&input, 0).unwrap();
    b.iter(|| symbol.split().last());
}

#[bench]
fn symbol_split_mixed_short(b: &mut test::Bencher) {
    let input = "abcABCAbc123";
    let symbol = defenestrate::tokens::Identifier::new(input, 0).unwrap();
    b.iter(|| symbol.split().last());
}

#[bench]
fn symbol_split_mixed_long(b: &mut test::Bencher) {
    let input = "abcABCAbc123".repeat(90);
    let symbol = defenestrate::tokens::Identifier::new(&input, 0).unwrap();
    b.iter(|| symbol.split().last());
}
