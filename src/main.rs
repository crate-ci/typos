// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::io::Write;

use structopt::StructOpt;

mod args;
use typos_cli::checks;
use typos_cli::config;
use typos_cli::dict;
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

    if let Some(output_path) = args.dump_config.as_ref() {
        run_dump_config(&args, output_path)
    } else {
        run_checks(&args)
    }
}

fn run_dump_config(args: &args::Args, output_path: &std::path::Path) -> proc_exit::ExitResult {
    let global_cwd = std::env::current_dir()?;

    let path = &args.path[0];
    let path = if path == std::path::Path::new("-") {
        path.to_owned()
    } else {
        path.canonicalize().with_code(proc_exit::Code::USAGE_ERR)?
    };
    let cwd = if path == std::path::Path::new("-") {
        global_cwd.as_path()
    } else if path.is_file() {
        path.parent().unwrap()
    } else {
        path.as_path()
    };

    let config = load_config(cwd, &args).with_code(proc_exit::Code::CONFIG_ERR)?;
    let mut defaulted_config = config::Config::from_defaults();
    defaulted_config.update(&config);
    let output = toml::to_string_pretty(&defaulted_config).with_code(proc_exit::Code::FAILURE)?;
    if output_path == std::path::Path::new("-") {
        std::io::stdout().write_all(output.as_bytes())?;
    } else {
        std::fs::write(output_path, &output)?;
    }

    Ok(())
}

fn run_checks(args: &args::Args) -> proc_exit::ExitResult {
    let global_cwd = std::env::current_dir()?;

    let mut typos_found = false;
    let mut errors_found = false;
    for path in args.path.iter() {
        let path = if path == std::path::Path::new("-") {
            path.to_owned()
        } else {
            path.canonicalize().with_code(proc_exit::Code::USAGE_ERR)?
        };
        let cwd = if path == std::path::Path::new("-") {
            global_cwd.as_path()
        } else if path.is_file() {
            path.parent().unwrap()
        } else {
            path.as_path()
        };
        let config = load_config(cwd, &args).with_code(proc_exit::Code::CONFIG_ERR)?;

        let tokenizer = typos::tokens::TokenizerBuilder::new()
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

        let mut settings = checks::CheckSettings::new();
        settings
            .check_filenames(config.default.check_filename())
            .check_files(config.default.check_file())
            .binary(config.default.binary());

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
        let reporter: &dyn report::Report = &status_reporter;

        let selected_checks: &dyn checks::FileChecker = if args.files {
            &checks::FoundFiles
        } else if args.identifiers {
            &checks::Identifiers
        } else if args.words {
            &checks::Words
        } else if args.write_changes {
            &checks::FixTypos
        } else if args.diff {
            &checks::DiffTypos
        } else {
            &checks::Typos
        };

        if single_threaded {
            checks::walk_path(
                walk.build(),
                selected_checks,
                &settings,
                &tokenizer,
                &dictionary,
                reporter,
            )
        } else {
            checks::walk_path_parallel(
                walk.build_parallel(),
                selected_checks,
                &settings,
                &tokenizer,
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

fn load_config(cwd: &std::path::Path, args: &args::Args) -> Result<config::Config, anyhow::Error> {
    let mut config = config::Config::default();

    if !args.isolated {
        let derived = config::Config::derive(cwd)?;
        config.update(&derived);
    }
    if let Some(path) = args.custom_config.as_ref() {
        config.update(&config::Config::from_file(path)?);
    }

    config.update(&args.config);
    config.default.update(&args.overrides);

    Ok(config)
}
