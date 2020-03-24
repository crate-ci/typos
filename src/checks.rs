use std::sync::atomic;

pub(crate) trait Checks: Send + Sync {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error>;

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error>;
}

impl<'p> Checks for typos::checks::ParseIdentifiers {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, report)
    }
}

impl<'p> Checks for typos::checks::ParseWords {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        _dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, report)
    }
}

impl<'d, 'p> Checks for typos::checks::Checks {
    fn check_filename(
        &self,
        path: &std::path::Path,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_filename(path, parser, dictionary, report)
    }

    fn check_file(
        &self,
        path: &std::path::Path,
        explicit: bool,
        parser: &typos::tokens::Parser,
        dictionary: &dyn typos::Dictionary,
        report: &dyn typos::report::Report,
    ) -> Result<bool, typos::Error> {
        self.check_file(path, explicit, parser, dictionary, report)
    }
}

pub(crate) fn check_path(
    walk: ignore::Walk,
    checks: &dyn Checks,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn typos::report::Report,
) -> (bool, bool) {
    let mut typos_found = false;
    let mut errors_found = false;

    for entry in walk {
        match check_entry(entry, checks, parser, dictionary, reporter) {
            Ok(true) => typos_found = true,
            Err(err) => {
                let msg = typos::report::Error::new(err.to_string());
                reporter.report(msg.into());
                errors_found = true
            }
            _ => (),
        }
    }

    (typos_found, errors_found)
}

pub(crate) fn check_path_parallel(
    walk: ignore::WalkParallel,
    checks: &dyn Checks,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn typos::report::Report,
) -> (bool, bool) {
    let typos_found = atomic::AtomicBool::new(false);
    let errors_found = atomic::AtomicBool::new(false);

    walk.run(|| {
        Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
            match check_entry(entry, checks, parser, dictionary, reporter) {
                Ok(true) => typos_found.store(true, atomic::Ordering::Relaxed),
                Err(err) => {
                    let msg = typos::report::Error::new(err.to_string());
                    reporter.report(msg.into());
                    errors_found.store(true, atomic::Ordering::Relaxed);
                }
                _ => (),
            }
            ignore::WalkState::Continue
        })
    });

    (typos_found.into_inner(), errors_found.into_inner())
}

fn check_entry(
    entry: Result<ignore::DirEntry, ignore::Error>,
    checks: &dyn Checks,
    parser: &typos::tokens::Parser,
    dictionary: &dyn typos::Dictionary,
    reporter: &dyn typos::report::Report,
) -> Result<bool, anyhow::Error> {
    let mut typos_found = false;

    let entry = entry?;
    if entry.file_type().map(|t| t.is_file()).unwrap_or(true) {
        let explicit = entry.depth() == 0;
        if checks.check_filename(entry.path(), parser, dictionary, reporter)? {
            typos_found = true;
        }
        if checks.check_file(entry.path(), explicit, parser, dictionary, reporter)? {
            typos_found = true;
        }
    }

    Ok(typos_found)
}
