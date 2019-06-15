#![feature(test)]

extern crate test;

mod data;

#[bench]
fn symbol_parse_empty(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Symbol::parse(data::EMPTY.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn symbol_parse_no_tokens(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Symbol::parse(data::NO_TOKENS.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn symbol_parse_single_token(b: &mut test::Bencher) {
    b.iter(|| {
        defenestrate::tokens::Symbol::parse(data::SINGLE_TOKEN.as_bytes()).collect::<Vec<_>>()
    });
}

#[bench]
fn symbol_parse_sherlock(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Symbol::parse(data::SHERLOCK.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn symbol_parse_code(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Symbol::parse(data::CODE.as_bytes()).collect::<Vec<_>>());
}

#[bench]
fn symbol_parse_corpus(b: &mut test::Bencher) {
    b.iter(|| defenestrate::tokens::Symbol::parse(data::CORPUS.as_bytes()).collect::<Vec<_>>());
}
