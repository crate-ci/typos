//! Regression tests for <https://github.com/crate-ci/typos/issues/1444>
//!
//! `walk_entry` canonicalizes each entry's path to look up its policy. When that failed,
//! `report_result` reported the error and then handed back `PathBuf::default()`, an empty
//! path that is never a key in `ConfigEngine`'s directory map, so the `policy()` call
//! right after it panicked with `` `walk()` should be called first ``.
//!
//! The panic is gone, but a file that a concurrent process removes mid-scan is still reported
//! as an error at each of the two points it can vanish: the `canonicalize()` above, and the
//! read in `check_file` after it. `main` turns any reported error into a failing exit code, so
//! an unrelated tool dropping a temporary file is enough to fail an otherwise clean run.
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
fn walk_path_reports_file_removed_mid_walk() {
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
    assert!(
        !reporter.errors.lock().unwrap().is_empty(),
        "expected the vanished file's canonicalize() failure to be reported"
    );
}

#[test]
fn check_file_reports_file_removed_before_read() {
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
    assert!(
        !reporter.errors.lock().unwrap().is_empty(),
        "expected the vanished file's read() failure to be reported"
    );
}
