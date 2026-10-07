//! Regression tests for <https://github.com/crate-ci/typos/issues/1444>
//!
//! A file removed by another process mid-scan can vanish at either of two points. `walk_entry`
//! canonicalizes each entry's path to look up its policy, and `check_file` reads the file
//! afterwards; both used to report a `NotFound` as an error, and `main` turns any reported error
//! into a failing exit code. A tool that drops a temporary file next to the sources being scanned
//! was therefore enough to fail an otherwise clean run.
#![cfg(unix)]

use typos_cli::file::FileChecker;

struct CollectingReporter {
    errors: std::sync::Mutex<Vec<String>>,
}

impl CollectingReporter {
    fn new() -> Self {
        Self {
            errors: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl typos_cli::report::Report for CollectingReporter {
    fn report(&self, msg: typos_cli::report::Message<'_>) -> Result<(), std::io::Error> {
        if msg.is_error() {
            self.errors.lock().unwrap().push(format!("{msg:?}"));
        }
        Ok(())
    }
}

#[test]
fn walk_path_skips_file_removed_before_canonicalize() {
    let temp = assert_fs::TempDir::new().unwrap();
    let vanishing = temp.path().join("vanishing.txt");
    std::fs::write(&vanishing, b"helllo world\n").unwrap();

    let storage = typos_cli::policy::ConfigStorage::new();
    let mut engine = typos_cli::policy::ConfigEngine::new(&storage);
    engine.set_isolated(true);
    engine.set_overrides(typos_cli::config::Config::default());
    let cwd = temp.path().canonicalize().unwrap();
    engine.init_dir(&cwd).unwrap();

    // Remove the file as the walker visits it. `ignore` still yields it as an `Ok` entry,
    // but the `canonicalize()` in `walk_entry` then fails with `NotFound` -- the same
    // outcome as a file that a concurrent process removes mid-scan.
    let mut builder = ignore::WalkBuilder::new(temp.path());
    let target = vanishing.clone();
    let removed = std::sync::atomic::AtomicBool::new(false);
    builder.filter_entry(move |entry| {
        if entry.path() == target && !removed.swap(true, std::sync::atomic::Ordering::SeqCst) {
            std::fs::remove_file(&target).unwrap();
        }
        true
    });
    let reporter = CollectingReporter::new();

    let result = typos_cli::file::walk_path(
        builder.build(),
        &typos_cli::file::Typos,
        &engine,
        &reporter,
        false,
    );

    assert!(
        result.is_ok(),
        "walk_path should not surface an ignore::Error: {result:?}"
    );
    let errors = reporter.errors.lock().unwrap();
    assert!(
        errors.is_empty(),
        "the vanished path should be skipped rather than reported: {errors:?}"
    );
}

#[test]
fn check_file_skips_file_removed_before_read() {
    let temp = assert_fs::TempDir::new().unwrap();

    let storage = typos_cli::policy::ConfigStorage::new();
    let mut engine = typos_cli::policy::ConfigEngine::new(&storage);
    engine.set_isolated(true);
    engine.set_overrides(typos_cli::config::Config::default());
    let cwd = temp.path().canonicalize().unwrap();
    engine.init_dir(&cwd).unwrap();

    // The walk canonicalizes a path before handing it to `check_file`, so a file removed between
    // the two is one that is simply no longer there when `read_file` opens it. The walk cannot be
    // driven into that window on demand -- `ignore` runs `filter_entry` for the next entry only
    // once the current one has been through `walk_entry` -- so the checker is called directly.
    let vanished = cwd.join("vanished.txt");
    let policy = engine.policy(&vanished);
    let reporter = CollectingReporter::new();

    let result = typos_cli::file::Typos.check_file(&vanished, false, &policy, &reporter);

    assert!(
        result.is_ok(),
        "check_file should not surface an io::Error: {result:?}"
    );
    let errors = reporter.errors.lock().unwrap();
    assert!(
        errors.is_empty(),
        "the vanished file should be skipped rather than reported: {errors:?}"
    );
}
