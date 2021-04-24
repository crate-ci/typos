mod data;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_tokenize(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenize");
    for (name, sample) in data::DATA {
        let len = sample.len();
        group.bench_with_input(BenchmarkId::new("ident(bytes)", name), &len, |b, _| {
            let parser = typos::tokens::Tokenizer::new();
            b.iter(|| parser.parse_bytes(sample.as_bytes()).last());
        });
        group.bench_with_input(BenchmarkId::new("ident(str)", name), &len, |b, _| {
            let parser = typos::tokens::Tokenizer::new();
            b.iter(|| parser.parse_str(sample).last());
        });
        group.bench_with_input(BenchmarkId::new("words", name), &len, |b, _| {
            let symbol =
                typos::tokens::Identifier::new_unchecked(sample, typos::tokens::Case::None, 0);
            b.iter(|| symbol.split().last());
        });
        group.bench_with_input(
            BenchmarkId::new("ident(bytes)+words", name),
            &len,
            |b, _| {
                let parser = typos::tokens::Tokenizer::new();
                b.iter(|| {
                    parser
                        .parse_bytes(sample.as_bytes())
                        .flat_map(|i| i.split())
                        .last()
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_tokenize);
criterion_main!(benches);
