#![feature(test)]

extern crate test;

#[bench]
fn load_corrections(b: &mut test::Bencher) {
    b.iter(|| typos_cli::dict::BuiltIn::new(Default::default()));
}

#[bench]
fn correct_word_hit(b: &mut test::Bencher) {
    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let input = typos::tokens::Word::new("successs", 0).unwrap();
    assert_eq!(
        corrections.correct_word(input),
        vec![std::borrow::Cow::Borrowed("successes")]
    );
    b.iter(|| corrections.correct_word(input));
}

#[bench]
fn correct_word_miss(b: &mut test::Bencher) {
    let corrections = typos_cli::dict::BuiltIn::new(Default::default());
    let input = typos::tokens::Word::new("success", 0).unwrap();
    assert!(corrections.correct_word(input).is_empty());
    b.iter(|| corrections.correct_word(input));
}
