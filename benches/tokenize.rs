mod data;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_parse_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_str");
    for (name, sample) in data::DATA {
        let len = sample.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("unicode", name), &len, |b, _| {
            let parser = typos::tokens::TokenizerBuilder::new().unicode(true).build();
            b.iter(|| parser.parse_str(sample).last());
        });
        group.bench_with_input(BenchmarkId::new("ascii", name), &len, |b, _| {
            let parser = typos::tokens::TokenizerBuilder::new()
                .unicode(false)
                .build();
            b.iter(|| parser.parse_str(sample).last());
        });
    }
    group.finish();
}

fn bench_parse_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_bytes");
    for (name, sample) in data::DATA {
        let len = sample.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("unicode", name), &len, |b, _| {
            let parser = typos::tokens::TokenizerBuilder::new().unicode(true).build();
            b.iter(|| parser.parse_bytes(sample.as_bytes()).last());
        });
        group.bench_with_input(BenchmarkId::new("ascii", name), &len, |b, _| {
            let parser = typos::tokens::TokenizerBuilder::new()
                .unicode(false)
                .build();
            b.iter(|| parser.parse_bytes(sample.as_bytes()).last());
        });
    }
    group.finish();
}

fn bench_split(c: &mut Criterion) {
    let mut group = c.benchmark_group("split");
    for (name, sample) in data::DATA {
        let len = sample.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("words", name), &len, |b, _| {
            let symbol =
                typos::tokens::Identifier::new_unchecked(sample, typos::tokens::Case::None, 0);
            b.iter(|| symbol.split().last());
        });
    }
    group.finish();
}

fn bench_parse_split(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_bytes+split");
    for (name, sample) in data::DATA {
        let len = sample.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("unicode", name), &len, |b, _| {
            let parser = typos::tokens::TokenizerBuilder::new().unicode(true).build();
            b.iter(|| {
                parser
                    .parse_bytes(sample.as_bytes())
                    .flat_map(|i| i.split())
                    .last()
            });
        });
        group.bench_with_input(BenchmarkId::new("ascii", name), &len, |b, _| {
            let parser = typos::tokens::TokenizerBuilder::new()
                .unicode(false)
                .build();
            b.iter(|| {
                parser
                    .parse_bytes(sample.as_bytes())
                    .flat_map(|i| i.split())
                    .last()
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_str,
    bench_parse_bytes,
    bench_split,
    bench_parse_split
);
criterion_main!(benches);
