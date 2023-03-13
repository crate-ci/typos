use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_dict_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("load");
    group.bench_function(BenchmarkId::new("load", "builtin"), |b| {
        b.iter(|| typos_cli::dict::BuiltIn::new(Default::default()));
    });
    group.finish();
}

fn bench_dict_correct_word(c: &mut Criterion) {
    let mut group = c.benchmark_group("correct_word");

    {
        let case = "ok";
        let input = "finalizes";
        group.bench_function(BenchmarkId::new("en", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::En);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            #[cfg(feature = "vars")]
            assert_eq!(corrections.correct_word(input), None);
            b.iter(|| corrections.correct_word(input));
        });
        #[cfg(feature = "vars")]
        group.bench_function(BenchmarkId::new("en-us", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::EnUs);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            #[cfg(feature = "vars")]
            assert_eq!(corrections.correct_word(input), Some(typos::Status::Valid));
            b.iter(|| corrections.correct_word(input));
        });
    }
    {
        let case = "misspell";
        let input = "finallizes";
        let output = "finalizes";
        group.bench_function(BenchmarkId::new("en", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::En);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            b.iter(|| corrections.correct_word(input));
        });
        #[cfg(feature = "vars")]
        group.bench_function(BenchmarkId::new("en-us", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::EnUs);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            b.iter(|| corrections.correct_word(input));
        });
    }
    {
        let case = "misspell_case";
        let input = "FINALLIZES";
        let output = "FINALIZES";
        group.bench_function(BenchmarkId::new("en", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::En);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            b.iter(|| corrections.correct_word(input));
        });
    }
    #[cfg(feature = "vars")]
    {
        let case = "varcon";
        let input = "finalizes";
        let output = "finalises";
        group.bench_function(BenchmarkId::new("en-gb", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::EnGb);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            b.iter(|| corrections.correct_word(input));
        });
    }
    #[cfg(feature = "vars")]
    {
        let case = "misspell_varcon";
        let input = "finallizes";
        let output = "finalises";
        group.bench_function(BenchmarkId::new("en-gb", case), |b| {
            let corrections = typos_cli::dict::BuiltIn::new(typos_cli::config::Locale::EnGb);
            let input = typos::tokens::Word::new(input, 0).unwrap();
            assert_eq!(
                corrections.correct_word(input),
                Some(typos::Status::Corrections(vec![
                    std::borrow::Cow::Borrowed(output)
                ]))
            );
            b.iter(|| corrections.correct_word(input));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_dict_load, bench_dict_correct_word);
criterion_main!(benches);
