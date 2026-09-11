//! Regression test for <https://github.com/crate-ci/typos/issues/1444>
//!
//! `walk_entry` canonicalizes each entry's path to look up its policy. When that fails,
//! `report_result` reports the error and then hands back `PathBuf::default()`, an empty
//! path that is never a key in `ConfigEngine`'s directory map, so the `policy()` call
//! right after it panics with `` `walk()` should be called first ``.
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

    let payload = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        typos_cli::file::walk_path(
            builder.build(),
            &typos_cli::file::Typos,
            &engine,
            &reporter,
            false,
        )
    }))
    .expect_err("walk_path should panic on the failed policy lookup");
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied());
    // `policy()` checks the path with a `debug_assert!` before reaching its `expect`.
    let expected = if cfg!(debug_assertions) {
        " is not absolute"
    } else {
        "`walk()` should be called first"
    };
    assert_eq!(message, Some(expected));
    assert!(
        !reporter.errors.lock().unwrap().is_empty(),
        "expected the vanished file's canonicalize() failure to be reported"
    );
}
