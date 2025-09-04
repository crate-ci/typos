#![allow(clippy::needless_update)]

use std::borrow::Cow;
use std::io::Write as _;
use std::ops::Range;
use std::sync::{atomic, Mutex};

use annotate_snippets::Annotation;
use annotate_snippets::AnnotationKind;
use annotate_snippets::Group;
use annotate_snippets::Level;
use annotate_snippets::Origin;
use annotate_snippets::Snippet;
use anstream::stderr;
use anstream::stdout;
use serde_sarif::sarif;
use serde_sarif::sarif::{ArtifactChange, ArtifactContent, Fix, Replacement};
use typos_cli::report::{Context, Error, Message, Report, Typo};

const INFO: anstyle::Style = anstyle::AnsiColor::BrightBlue.on_default();

pub(crate) struct MessageStatus<'r> {
    typos_found: atomic::AtomicBool,
    errors_found: atomic::AtomicBool,
    reporter: &'r dyn Report,
}

impl<'r> MessageStatus<'r> {
    pub(crate) fn new(reporter: &'r dyn Report) -> Self {
        Self {
            typos_found: atomic::AtomicBool::new(false),
            errors_found: atomic::AtomicBool::new(false),
            reporter,
        }
    }

    pub(crate) fn typos_found(&self) -> bool {
        self.typos_found.load(atomic::Ordering::Relaxed)
    }

    pub(crate) fn errors_found(&self) -> bool {
        self.errors_found.load(atomic::Ordering::Relaxed)
    }
}

impl Report for MessageStatus<'_> {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error> {
        if msg.is_typo() {
            self.typos_found.store(true, atomic::Ordering::Relaxed);
        }
        if msg.is_error() {
            self.errors_found.store(true, atomic::Ordering::Relaxed);
        }
        self.reporter.report(msg)
    }

    fn generate_final_result(&self) -> Result<(), std::io::Error> {
        self.reporter.generate_final_result()
    }
}

#[derive(Debug, Default)]
pub(crate) struct PrintSilent;

impl Report for PrintSilent {
    fn report(&self, _msg: Message<'_>) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub(crate) struct PrintBrief;

impl Report for PrintBrief {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error> {
        let renderer = RENDERER.clone().short_message(true);
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{msg}");
            }
            Message::Typo(msg) => {
                let report = &[typo_to_group(msg)];
                writeln!(stdout(), "{}", renderer.render(report))?;
            }
            Message::FileType(msg) => {
                let info = INFO.render();
                let reset = anstyle::Reset.render();
                writeln!(
                    stdout().lock(),
                    "{info}{}{reset}: {}",
                    msg.path.display(),
                    msg.file_type.unwrap_or("-")
                )?;
            }
            Message::File(msg) => {
                writeln!(stdout().lock(), "{}", msg.path.display())?;
            }
            Message::Parse(msg) => {
                writeln!(stdout().lock(), "{}", msg.data)?;
            }
            Message::Error(msg) => {
                let report = &[error_to_group(msg)];
                writeln!(stderr(), "{}", renderer.render(report))?;
            }
            _ => unimplemented!("New message {:?}", msg),
        }
        Ok(())
    }
}

pub(crate) struct PrintLong;

impl Report for PrintLong {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error> {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{msg}");
            }
            Message::Typo(msg) => {
                let report = &[typo_to_group(msg)];
                writeln!(stdout(), "{}", RENDERER.render(report))?;
            }
            Message::FileType(msg) => {
                let info = INFO.render();
                let reset = anstyle::Reset.render();
                writeln!(
                    stdout().lock(),
                    "{info}{}{reset}: {}",
                    msg.path.display(),
                    msg.file_type.unwrap_or("-")
                )?;
            }
            Message::File(msg) => {
                writeln!(stdout().lock(), "{}", msg.path.display())?;
            }
            Message::Parse(msg) => {
                writeln!(stdout().lock(), "{}", msg.data)?;
            }
            Message::Error(msg) => {
                let report = &[error_to_group(msg)];
                writeln!(stderr(), "{}", RENDERER.render(report))?;
            }
            _ => unimplemented!("New message {:?}", msg),
        }
        Ok(())
    }
}

fn typo_to_group<'t>(msg: &'t Typo<'t>) -> Group<'t> {
    let title = match &msg.corrections {
        typos::Status::Valid => unimplemented!("never valid words to report"),
        typos::Status::Invalid => {
            format!("`{}` is disallowed", msg.typo,)
        }
        typos::Status::Corrections(corrections) => {
            format!(
                "`{}` should be {}",
                msg.typo,
                itertools::join(corrections.iter().map(|s| format!("`{s}`")), ", ")
            )
        }
    };
    let group = Group::with_title(Level::ERROR.primary_title(Cow::Owned(title)));
    let group = match &msg.context {
        Some(Context::File(context)) => {
            let path = context.path.as_os_str().to_string_lossy();
            let (line, span) = to_string(&msg.buffer, msg.byte_offset, msg.typo.len());
            let snippet = Snippet::source(line)
                .path(path)
                .line_start(context.line_num);
            append_corrections(span, snippet, group)
        }
        Some(Context::Path(context)) => {
            let parent = context.path.parent().unwrap_or(std::path::Path::new("."));
            let parent = parent.as_os_str().to_string_lossy();
            let mut line = parent.into_owned();
            line.push(std::path::MAIN_SEPARATOR);
            let parent_len = line.len();
            let mut line = line.into_bytes();
            line.extend(msg.buffer.iter());
            let (line, span) = to_string(&line, parent_len + msg.byte_offset, msg.typo.len());
            let line = line.into_owned();
            let snippet = Snippet::source(line);
            append_corrections(span, snippet, group)
        }
        Some(_) | None => group,
    };
    group
}

fn append_corrections<'t>(
    span: Range<usize>,
    snippet: Snippet<'t, Annotation<'t>>,
    group: Group<'t>,
) -> Group<'t> {
    let snippet = snippet.annotation(AnnotationKind::Primary.span(span));
    group.element(snippet)
}

fn to_string(line: &[u8], start: usize, len: usize) -> (Cow<'_, str>, Range<usize>) {
    let end = start + len;

    if let Ok(line) = std::str::from_utf8(line) {
        return (Cow::Borrowed(line), start..end);
    }

    let prefix = &line[0..start];
    let prefix = String::from_utf8_lossy(prefix);

    let middle = &line[start..end];
    let middle = String::from_utf8_lossy(middle);

    let suffix = &line[end..];
    let suffix = String::from_utf8_lossy(suffix);

    let span_start = prefix.len();
    let span_end = span_start + middle.len();

    (
        Cow::Owned(format!("{prefix}{middle}{suffix}")),
        span_start..span_end,
    )
}

fn error_to_group<'e>(error: &'e Error<'e>) -> Group<'e> {
    let group = Group::with_title(Level::ERROR.primary_title(&error.msg));
    match &error.context {
        Some(Context::File(context)) => group.element(
            Origin::path(context.path.as_os_str().to_string_lossy()).line(context.line_num),
        ),
        Some(Context::Path(context)) => {
            group.element(Origin::path(context.path.as_os_str().to_string_lossy()))
        }
        Some(_) | None => group,
    }
}

static RENDERER: std::sync::LazyLock<annotate_snippets::Renderer> =
    std::sync::LazyLock::new(|| {
        let width = terminal_size::terminal_size()
            .map(|(w, _)| w.0 as usize)
            .unwrap_or(annotate_snippets::renderer::DEFAULT_TERM_WIDTH);
        let decor_style = if supports_unicode::supports_unicode() {
            annotate_snippets::renderer::DecorStyle::Unicode
        } else {
            annotate_snippets::renderer::DecorStyle::Ascii
        };
        annotate_snippets::Renderer::styled()
            .term_width(width)
            .decor_style(decor_style)
    });

#[derive(Copy, Clone, Debug)]
pub(crate) struct PrintJson;

impl Report for PrintJson {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error> {
        writeln!(stdout().lock(), "{}", serde_json::to_string(&msg).unwrap())?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PrintSarif {
    results: Mutex<Vec<sarif::Result>>,
    error: Mutex<Vec<String>>,
}

impl Default for PrintSarif {
    fn default() -> Self {
        Self {
            results: Mutex::new(Vec::new()),
            error: Mutex::new(Vec::new()),
        }
    }
}

impl Report for PrintSarif {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error> {
        self.report_sarif(msg).map_err(sarif_error_mapper)
    }

    fn generate_final_result(&self) -> Result<(), std::io::Error> {
        self.generate_final_result().map_err(sarif_error_mapper)
    }
}

impl PrintSarif {
    fn report_sarif(&self, msg: Message<'_>) -> Result<(), Box<dyn std::error::Error>> {
        match &msg {
            Message::Typo(msg) => {
                if msg.corrections.is_valid() {
                    return Ok(());
                }
                let message = type_to_sarif_message(msg).unwrap();
                let location = typo_to_sarif_location(msg)?;

                let fix =
                    typo_to_sarif_fix(message.clone(), msg.corrections.clone(), location.clone())?;
                let result = typo_to_sarif_result(message, location, fix)?;

                self.results.lock().unwrap().push(result);
            }
            Message::Error(msg) => {
                self.error.lock().unwrap().push(msg.msg.clone());
            }
            Message::BinaryFile(_) => {}
            Message::Parse(_) | Message::FileType(_) | Message::File(_) => {}
            _ => unimplemented!("New message {:?}", msg),
        }

        Ok(())
    }

    fn generate_final_result(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tool = sarif::Tool::builder()
            .driver(
                sarif::ToolComponent::builder()
                    .name("typos")
                    .information_uri(env!("CARGO_PKG_REPOSITORY"))
                    .build(),
            )
            .build();

        let run_builder = sarif::Run::builder()
            .tool(tool)
            .column_kind(sarif::ResultColumnKind::UnicodeCodePoints.to_string())
            .results(self.results.lock().unwrap().clone());

        let run = if !self.error.lock().unwrap().is_empty() {
            let invocations = self
                .error
                .lock()
                .unwrap()
                .iter()
                .map(|x| {
                    sarif::Invocation::builder()
                        .execution_successful(false)
                        .process_start_failure_message(x.clone())
                        .build()
                })
                .collect::<Vec<_>>();

            let run_builder = run_builder.invocations(invocations);
            run_builder.build()
        } else {
            run_builder.build()
        };

        let sarif_builder = sarif::Sarif::builder()
            .version(sarif::Version::V2_1_0.to_string())
            .schema(sarif::SCHEMA_URL)
            .runs(vec![run]);

        let sarif = sarif_builder.build();

        serde_json::to_writer_pretty(stdout().lock(), &sarif)?;

        Ok(())
    }
}

fn sarif_error_mapper(error: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::other(format!("failed to generate SARIF output: {error}"))
}

fn typo_to_sarif_result(
    message: String,
    location: sarif::Location,
    fix: Option<Fix>,
) -> Result<sarif::Result, Box<dyn std::error::Error>> {
    let mut result = sarif::Result::builder()
        .level(sarif::ResultLevel::Error.to_string())
        .message(sarif::Message::builder().markdown(message).build())
        .locations(vec![location])
        .build();
    if let Some(fix) = fix {
        result.fixes = Some(vec![fix]);
    }
    Ok(result)
}

fn typo_to_sarif_fix(
    message: String,
    correct: typos::Status<'_>,
    location: sarif::Location,
) -> Result<Option<Fix>, Box<dyn std::error::Error>> {
    let physical_location = location.physical_location.unwrap();
    let Some(region) = physical_location.region else {
        return Ok(None);
    };

    let mut replacements = vec![];

    match correct {
        typos::Status::Corrections(corrections) => {
            for correction in corrections.iter() {
                replacements.push(
                    Replacement::builder()
                        .deleted_region(region.clone())
                        .inserted_content(
                            ArtifactContent::builder().text(correction.clone()).build(),
                        )
                        .build(),
                );
            }
        }
        _ => return Ok(None),
    }

    let change = ArtifactChange::builder()
        .artifact_location(physical_location.artifact_location.unwrap())
        .replacements(replacements)
        .build();

    let fix = Fix::builder()
        .description(sarif::Message::builder().markdown(message).build())
        .artifact_changes(vec![change])
        .build();

    Ok(Some(fix))
}

fn type_to_sarif_message(msg: &Typo<'_>) -> Option<String> {
    match &msg.corrections {
        typos::Status::Valid => None,
        typos::Status::Invalid => Some(format!("`{}` is disallowed", msg.typo)),
        typos::Status::Corrections(corrections) => Some(format!(
            "`{}` should be {}",
            msg.typo,
            itertools::join(corrections.iter().map(|s| format!("`{s}`")), ", ",)
        )),
    }
}

fn typo_to_sarif_location(msg: &Typo<'_>) -> Result<sarif::Location, Box<dyn std::error::Error>> {
    let path = match &msg.context {
        Some(Context::File(ctx)) => ctx.path,
        Some(Context::Path(ctx)) => ctx.path,
        None => std::path::Path::new(""),
        _ => unimplemented!("New context {:?}", msg),
    };

    let artifact = sarif::ArtifactLocation::builder()
        .uri(
            path.display()
                .to_string()
                .replace(std::path::MAIN_SEPARATOR, "/"),
        )
        .build();
    let physical = sarif::PhysicalLocation::builder().artifact_location(artifact);

    if let Some(Context::File(context)) = &msg.context {
        let start = String::from_utf8_lossy(&msg.buffer[0..msg.byte_offset]);
        let column_start = start.chars().count() + 1;
        let column_end = msg.typo.chars().count() + column_start;
        let line_num = context.line_num;

        let physical = physical.region(
            sarif::Region::builder()
                .start_line(line_num as i64)
                .end_line(line_num as i64)
                .start_column(column_start as i64)
                .end_column(column_end as i64)
                .build(),
        );
        let location = sarif::Location::builder()
            .physical_location(physical.build())
            .build();
        Ok(location)
    } else {
        let location = sarif::Location::builder()
            .physical_location(physical.build())
            .build();
        Ok(location)
    }
}
