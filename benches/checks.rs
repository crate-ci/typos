mod data;

use assert_fs::prelude::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use typos_cli::file::FileChecker;

fn bench_checks(c: &mut Criterion) {
    let dict = typos_cli::dict::BuiltIn::new(Default::default());
    let tokenizer = typos::tokens::Tokenizer::new();
    let policy = typos_cli::policy::Policy::new()
        .dict(&dict)
        .tokenizer(&tokenizer);

    let temp = assert_fs::TempDir::new().unwrap();

    let mut group = c.benchmark_group("check_file");
    for (name, sample) in data::DATA {
        let sample_path = temp.child(name);
        sample_path.write_str(sample).unwrap();

        let len = sample.len();
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("FoundFiles", name), &len, |b, _| {
            b.iter(|| {
                typos_cli::file::FoundFiles.check_file(
                    sample_path.path(),
                    true,
                    &policy,
                    &PrintSilent,
                )
            });
        });
        group.bench_with_input(BenchmarkId::new("Identifiers", name), &len, |b, _| {
            b.iter(|| {
                typos_cli::file::Identifiers.check_file(
                    sample_path.path(),
                    true,
                    &policy,
                    &PrintSilent,
                )
            });
        });
        group.bench_with_input(BenchmarkId::new("Words", name), &len, |b, _| {
            b.iter(|| {
                typos_cli::file::Words.check_file(sample_path.path(), true, &policy, &PrintSilent)
            });
        });
        group.bench_with_input(BenchmarkId::new("Typos", name), &len, |b, _| {
            b.iter(|| {
                typos_cli::file::Typos.check_file(sample_path.path(), true, &policy, &PrintSilent)
            });
        });
    }
    group.finish();

    temp.close().unwrap();
}

#[derive(Debug, Default)]
pub struct PrintSilent;

impl typos_cli::report::Report for PrintSilent {
    fn report(&self, _msg: typos_cli::report::Message) -> Result<(), std::io::Error> {
        Ok(())
    }
}

criterion_group!(benches, bench_checks,);
criterion_main!(benches);
