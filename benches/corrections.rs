#![feature(test)]

extern crate test;

#[bench]
fn load_corrections(b: &mut test::Bencher) {
    b.iter(|| defenestrate::Dictionary::new());
}

#[bench]
fn correction(b: &mut test::Bencher) {
    let corrections = defenestrate::Dictionary::new();
    assert_eq!(corrections.correct_str("successs"), Some("successes"));
    b.iter(|| corrections.correct_str("successs"));
}

#[bench]
fn no_correction(b: &mut test::Bencher) {
    let corrections = defenestrate::Dictionary::new();
    assert_eq!(corrections.correct_str("success"), None);
    b.iter(|| corrections.correct_str("success"));
}
