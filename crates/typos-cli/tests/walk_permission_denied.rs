//! Regression test for <https://github.com/crate-ci/typos/issues/1444>
//!
//! `walk_entry` calls `Path::canonicalize()` on every walked entry to look up its policy.
//! If a file disappears between the `ignore` walker listing it and `canonicalize()` running
//! on it (e.g. a temp file removed mid-scan, as in issue #1535) -- or more generally
//! whenever `canonicalize()` fails for a reason the walker itself didn't already catch --
//! the error used to be reported but then silently swallowed into an empty, non-absolute
//! `PathBuf`. That bogus path is never a key in `ConfigEngine`'s directory map, so the
//! following `engine.policy(..)` call panicked with `` `walk()` should be called first ``.
#![cfg(unix)]

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
fn walk_path_survives_file_removed_mid_walk() {
    let temp = assert_fs::TempDir::new().unwrap();
    let vanishing = temp.path().join("vanishing.txt");
    std::fs::write(&vanishing, b"helllo world\n").unwrap();

    let storage = typos_cli::policy::ConfigStorage::new();
    let mut engine = typos_cli::policy::ConfigEngine::new(&storage);
    engine.set_isolated(true);
    engine.set_overrides(typos_cli::config::Config::default());
    let cwd = temp.path().canonicalize().unwrap();
    engine.init_dir(&cwd).unwrap();

    // Delete the file right as the walker visits it, so it's still an `Ok` entry from
    // `ignore`, but `canonicalize()` inside `walk_entry` fails with `NotFound` -- the same
    // race a concurrently-modified working tree can trigger.
    let mut builder = ignore::WalkBuilder::new(temp.path());
    let target = vanishing.clone();
    let already_deleted = std::sync::atomic::AtomicBool::new(false);
    builder.filter_entry(move |entry| {
        if entry.path() == target
            && !already_deleted.swap(true, std::sync::atomic::Ordering::SeqCst)
        {
            std::fs::remove_file(&target).unwrap();
        }
        true
    });
    let walk = builder.build();
    let reporter = CollectingReporter::new();

    // This used to panic with `` `walk()` should be called first `` instead of returning.
    let result =
        typos_cli::file::walk_path(walk, &typos_cli::file::Typos, &engine, &reporter, false);

    assert!(
        result.is_ok(),
        "walk_path should not surface an ignore::Error: {result:?}"
    );
    assert!(
        !reporter.errors.lock().unwrap().is_empty(),
        "expected the vanished file's canonicalize() failure to be reported"
    );
}
