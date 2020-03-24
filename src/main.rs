// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::io::Write;
use std::sync::atomic;

use structopt::StructOpt;

mod args;
mod checks;
mod config;
mod dict;

fn init_logging(level: Option<log::Level>) {
    if let Some(level) = level {
        let mut builder = env_logger::Builder::new();

        builder.filter(None, level.to_level_filter());

        if level == log::LevelFilter::Trace {
            builder.format_timestamp_secs();
        } else {
            builder.format(|f, record| {
                writeln!(
                    f,
                    "[{}] {}",
                    record.level().to_string().to_lowercase(),
                    record.args()
                )
            });
        }

        builder.init();
    }
}

fn run() -> Result<i32, anyhow::Error> {
    let args = args::Args::from_args();

    init_logging(args.verbose.log_level());

    let config = if let Some(path) = args.custom_config.as_ref() {
        config::Config::from_file(path)?
    } else {
        config::Config::default()
    };

    let mut typos_found = false;
    let mut errors_found = false;
    for path in args.path.iter() {
        let path = path.canonicalize()?;
        let cwd = if path.is_file() {
            path.parent().unwrap()
        } else {
            path.as_path()
        };

        let mut config = config.clone();
        if !args.isolated {
            let derived = config::Config::derive(cwd)?;
            config.update(&derived);
        }
        config.update(&args.config);
        config.default.update(&args.overrides);
        let config = config;

        let parser = typos::tokens::ParserBuilder::new()
            .ignore_hex(config.default.ignore_hex())
            .leading_digits(config.default.identifier_leading_digits())
            .leading_chars(config.default.identifier_leading_chars().to_owned())
            .include_digits(config.default.identifier_include_digits())
            .include_chars(config.default.identifier_include_chars().to_owned())
            .build();

        let dictionary = crate::dict::BuiltIn::new();

        let mut settings = typos::checks::TyposSettings::new();
        settings
            .check_filenames(config.default.check_filename())
            .check_files(config.default.check_file())
            .binary(config.files.binary());

        let threads = if path.is_file() { 1 } else { args.threads };
        let single_threaded = threads == 1;

        let mut walk = ignore::WalkBuilder::new(path);
        walk.threads(args.threads)
            .hidden(config.files.ignore_hidden())
            .ignore(config.files.ignore_dot())
            .git_global(config.files.ignore_global())
            .git_ignore(config.files.ignore_vcs())
            .git_exclude(config.files.ignore_vcs())
            .parents(config.files.ignore_parent());

        let reporter = args.format.reporter();

        if args.files {
            if single_threaded {
                for entry in walk.build() {
                    match entry {
                        Ok(entry) => {
                            let msg = typos::report::File::new(entry.path());
                            reporter.report(msg.into());
                        }
                        Err(err) => {
                            let msg = typos::report::Error::new(err.to_string());
                            reporter.report(msg.into());
                            errors_found = true
                        }
                    }
                }
            } else {
                let atomic_errors = atomic::AtomicBool::new(errors_found);
                walk.build_parallel().run(|| {
                    Box::new(|entry: Result<ignore::DirEntry, ignore::Error>| {
                        match entry {
                            Ok(entry) => {
                                let msg = typos::report::File::new(entry.path());
                                reporter.report(msg.into());
                            }
                            Err(err) => {
                                let msg = typos::report::Error::new(err.to_string());
                                reporter.report(msg.into());
                                atomic_errors.store(true, atomic::Ordering::Relaxed);
                            }
                        }
                        ignore::WalkState::Continue
                    })
                });
                errors_found = atomic_errors.into_inner();
            }
        } else {
            let (identifier_parser, word_parser, checks);
            let selected_checks: &dyn checks::Checks = if args.identifiers {
                identifier_parser = settings.build_identifier_parser();
                &identifier_parser
            } else if args.words {
                word_parser = settings.build_word_parser();
                &word_parser
            } else {
                checks = settings.build_checks();
                &checks
            };

            let (cur_typos, cur_errors) = if single_threaded {
                checks::check_path(
                    walk.build(),
                    selected_checks,
                    &parser,
                    &dictionary,
                    reporter,
                )
            } else {
                checks::check_path_parallel(
                    walk.build_parallel(),
                    selected_checks,
                    &parser,
                    &dictionary,
                    reporter,
                )
            };
            if cur_typos {
                typos_found = true;
            }
            if cur_errors {
                errors_found = true;
            }
        }
    }

    if errors_found {
        Ok(2)
    } else if typos_found {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
