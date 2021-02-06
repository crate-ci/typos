use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_dict_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("load");
    group.bench_function(BenchmarkId::new("load", "builtin"), |b| {
        b.iter(|| typos_cli::dict::BuiltIn::new(Default::default()));
    });
    group.finish();
}

fn bench_dict_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup");
    group.bench_function(BenchmarkId::new("lookup", "hit"), |b| {
        let corrections = typos_cli::dict::BuiltIn::new(Default::default());
        let input = typos::tokens::Word::new("successs", 0).unwrap();
        assert_eq!(
            corrections.correct_word(input),
            Some(typos::Status::Corrections(vec![
                std::borrow::Cow::Borrowed("successes")
            ]))
        );
        b.iter(|| corrections.correct_word(input));
    });
    group.bench_function(BenchmarkId::new("lookup", "miss"), |b| {
        let corrections = typos_cli::dict::BuiltIn::new(Default::default());
        let input = typos::tokens::Word::new("success", 0).unwrap();
        assert!(corrections.correct_word(input).is_none());
        b.iter(|| corrections.correct_word(input));
    });
    group.finish();
}

criterion_group!(benches, bench_dict_load, bench_dict_lookup);
criterion_main!(benches);
