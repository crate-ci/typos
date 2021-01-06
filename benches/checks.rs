mod data;

use assert_fs::prelude::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use typos_cli::checks::FileChecker;

fn bench_checks(c: &mut Criterion) {
    let mut group = c.benchmark_group("checks");
    for (name, sample) in data::DATA {
        let len = sample.len();
        group.bench_with_input(BenchmarkId::new("files", name), &len, |b, _| {
            let temp = assert_fs::TempDir::new().unwrap();
            let sample_path = temp.child("sample");
            sample_path.write_str(sample).unwrap();

            let corrections = typos_cli::dict::BuiltIn::new(Default::default());
            let parser = typos::tokens::Tokenizer::new();
            let settings = typos_cli::checks::CheckSettings::new();
            b.iter(|| {
                typos_cli::checks::FoundFiles.check_file(
                    sample_path.path(),
                    true,
                    &settings,
                    &parser,
                    &corrections,
                    &typos_cli::report::PrintSilent,
                )
            });

            temp.close().unwrap();
        });
        group.bench_with_input(BenchmarkId::new("identifiers", name), &len, |b, _| {
            let temp = assert_fs::TempDir::new().unwrap();
            let sample_path = temp.child("sample");
            sample_path.write_str(sample).unwrap();

            let corrections = typos_cli::dict::BuiltIn::new(Default::default());
            let parser = typos::tokens::Tokenizer::new();
            let settings = typos_cli::checks::CheckSettings::new();
            b.iter(|| {
                typos_cli::checks::Identifiers.check_file(
                    sample_path.path(),
                    true,
                    &settings,
                    &parser,
                    &corrections,
                    &typos_cli::report::PrintSilent,
                )
            });

            temp.close().unwrap();
        });
        group.bench_with_input(BenchmarkId::new("words", name), &len, |b, _| {
            let temp = assert_fs::TempDir::new().unwrap();
            let sample_path = temp.child("sample");
            sample_path.write_str(sample).unwrap();

            let corrections = typos_cli::dict::BuiltIn::new(Default::default());
            let parser = typos::tokens::Tokenizer::new();
            let settings = typos_cli::checks::CheckSettings::new();
            b.iter(|| {
                typos_cli::checks::Words.check_file(
                    sample_path.path(),
                    true,
                    &settings,
                    &parser,
                    &corrections,
                    &typos_cli::report::PrintSilent,
                )
            });

            temp.close().unwrap();
        });
        group.bench_with_input(BenchmarkId::new("typos", name), &len, |b, _| {
            let temp = assert_fs::TempDir::new().unwrap();
            let sample_path = temp.child("sample");
            sample_path.write_str(sample).unwrap();

            let corrections = typos_cli::dict::BuiltIn::new(Default::default());
            let parser = typos::tokens::Tokenizer::new();
            let settings = typos_cli::checks::CheckSettings::new();
            b.iter(|| {
                typos_cli::checks::Typos.check_file(
                    sample_path.path(),
                    true,
                    &settings,
                    &parser,
                    &corrections,
                    &typos_cli::report::PrintSilent,
                )
            });

            temp.close().unwrap();
        });
    }
    group.finish();
}

criterion_group!(benches, bench_checks,);
criterion_main!(benches);
