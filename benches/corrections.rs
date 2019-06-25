#![feature(test)]

extern crate test;

#[bench]
fn load_corrections(b: &mut test::Bencher) {
    b.iter(|| defenestrate::Dictionary::new());
}

#[bench]
fn correct_word_hit(b: &mut test::Bencher) {
    let corrections = defenestrate::Dictionary::new();
    let input = defenestrate::tokens::Word::new("successs", 0).unwrap();
    assert_eq!(
        corrections.correct_word(input),
        Some(std::borrow::Cow::Borrowed("successes"))
    );
    b.iter(|| corrections.correct_word(input));
}

#[bench]
fn correct_word_miss(b: &mut test::Bencher) {
    let corrections = defenestrate::Dictionary::new();
    let input = defenestrate::tokens::Word::new("success", 0).unwrap();
    assert_eq!(corrections.correct_word(input), None);
    b.iter(|| corrections.correct_word(input));
}
