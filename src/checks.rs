pub(crate) fn check_path(
    walk: ignore::Walk,
    checks: &dyn typos::checks::Check,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn typos::report::Report,
) -> Result<(), anyhow::Error> {
    for entry in walk {
        check_entry(entry, checks, parser, dictionary, reporter)?;
    }
    Ok(())
}

pub(crate) fn check_path_parallel(
    walk: ignore::WalkParallel,
    checks: &dyn typos::checks::Check,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn typos::report::Report,
) -> Result<(), anyhow::Error> {
    let error: std::sync::Mutex<Result<(), anyhow::Error>> = std::sync::Mutex::new(Ok(()));
    walk.run(|| {
        Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
            match check_entry(entry, checks, parser, dictionary, reporter) {
                Ok(()) => ignore::WalkState::Continue,
                Err(err) => {
                    *error.lock().unwrap() = Err(err);
                    ignore::WalkState::Quit
                }
            }
        })
    });

    error.into_inner().unwrap()
}

fn check_entry(
    entry: Result<ignore::DirEntry, ignore::Error>,
    checks: &dyn typos::checks::Check,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn typos::report::Report,
) -> Result<(), anyhow::Error> {
    let entry = entry?;
    if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
        let explicit = entry.depth() == 0;
        checks.check_filename(entry.path(), parser, dictionary, reporter)?;
        checks.check_file(entry.path(), explicit, parser, dictionary, reporter)?;
    }

    Ok(())
}
