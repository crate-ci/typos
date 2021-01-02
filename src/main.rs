// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::io::Write;

use structopt::StructOpt;

mod args;
use typos_cli::checks;
use typos_cli::config;
use typos_cli::dict;
use typos_cli::diff;
use typos_cli::report;

use proc_exit::WithCodeResultExt;

fn main() {
    human_panic::setup_panic!();
    let result = run();
    proc_exit::exit(result);
}

fn run() -> proc_exit::ExitResult {
    // clap's `get_matches` uses Failure rather than Usage, so bypass it for `get_matches_safe`.
    let args = match args::Args::from_args_safe() {
        Ok(args) => args,
        Err(e) if e.use_stderr() => {
            return Err(proc_exit::Code::USAGE_ERR.with_message(e));
        }
        Err(e) => {
            writeln!(std::io::stdout(), "{}", e)?;
            return proc_exit::Code::SUCCESS.ok();
        }
    };

    init_logging(args.verbose.log_level());

    let config = if let Some(path) = args.custom_config.as_ref() {
        config::Config::from_file(path).with_code(proc_exit::Code::CONFIG_ERR)?
    } else {
        config::Config::default()
    };

    let mut typos_found = false;
    let mut errors_found = false;
    for path in args.path.iter() {
        let path = path.canonicalize().with_code(proc_exit::Code::USAGE_ERR)?;
        let cwd = if path.is_file() {
            path.parent().unwrap()
        } else {
            path.as_path()
        };

        let mut config = config.clone();
        if !args.isolated {
            let derived = config::Config::derive(cwd).with_code(proc_exit::Code::CONFIG_ERR)?;
            config.update(&derived);
        }
        config.update(&args.config);
        config.default.update(&args.overrides);
        let config = config;

        let parser = typos::tokens::TokenizerBuilder::new()
            .ignore_hex(config.default.ignore_hex())
            .leading_digits(config.default.identifier_leading_digits())
            .leading_chars(config.default.identifier_leading_chars().to_owned())
            .include_digits(config.default.identifier_include_digits())
            .include_chars(config.default.identifier_include_chars().to_owned())
            .build();

        let dictionary = crate::dict::BuiltIn::new(config.default.locale());
        let mut dictionary = crate::dict::Override::new(dictionary);
        dictionary.identifiers(config.default.extend_identifiers());
        dictionary.words(config.default.extend_words());

        let mut settings = checks::TyposSettings::new();
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

        // HACK: Diff doesn't handle mixing content
        let output_reporter = if args.diff {
            &args::PRINT_SILENT
        } else {
            args.format.reporter()
        };
        let status_reporter = report::MessageStatus::new(output_reporter);
        let mut reporter: &dyn report::Report = &status_reporter;
        let diff_reporter = diff::Diff::new(reporter);
        if args.diff {
            reporter = &diff_reporter;
        }

        let (files, identifier_parser, word_parser, checks, fixer);
        let selected_checks: &dyn checks::Check = if args.files {
            files = settings.build_files();
            &files
        } else if args.identifiers {
            identifier_parser = settings.build_identifier_parser();
            &identifier_parser
        } else if args.words {
            word_parser = settings.build_word_parser();
            &word_parser
        } else if args.write_changes {
            fixer = settings.build_fix_typos();
            &fixer
        } else {
            checks = settings.build_typos();
            &checks
        };

        if single_threaded {
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
        }
        .map_err(|e| {
            e.io_error()
                .map(|i| proc_exit::Code::from(i.kind()))
                .unwrap_or_default()
                .with_message(e)
        })?;
        if status_reporter.typos_found() {
            typos_found = true;
        }
        if status_reporter.errors_found() {
            errors_found = true;
        }

        if args.diff {
            diff_reporter.show().with_code(proc_exit::Code::FAILURE)?;
        }
    }

    if errors_found {
        proc_exit::Code::FAILURE.ok()
    } else if typos_found {
        // Can;'t use `Failure` since its so prevalent, it could be easy to get a
        // `Failure` from something else and get it mixed up with typos.
        //
        // Can't use DataErr or anything else an std::io::ErrorKind might map to.
        proc_exit::Code::UNKNOWN.ok()
    } else {
        proc_exit::Code::SUCCESS.ok()
    }
}

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
