#![allow(clippy::needless_update)]

use std::io::Write as _;
use std::sync::{atomic, Mutex};

use anstream::stdout;
use serde_sarif::sarif;
use serde_sarif::sarif::{ArtifactChange, ArtifactContent, Fix, Replacement};
use typos_cli::report::{Context, Message, Report, Typo};
use unicode_width::UnicodeWidthStr;

const ERROR: anstyle::Style = anstyle::AnsiColor::BrightRed.on_default();
const INFO: anstyle::Style = anstyle::AnsiColor::BrightBlue.on_default();
const GOOD: anstyle::Style = anstyle::AnsiColor::BrightGreen.on_default();

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
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{msg}");
            }
            Message::Typo(msg) => print_brief_correction(msg)?,
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
                log::error!("{}: {}", context_display(&msg.context), msg.msg);
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
            Message::Typo(msg) => print_long_correction(msg)?,
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
                log::error!("{}: {}", context_display(&msg.context), msg.msg);
            }
            _ => unimplemented!("New message {:?}", msg),
        }
        Ok(())
    }
}

fn print_brief_correction(msg: &Typo<'_>) -> Result<(), std::io::Error> {
    let error = ERROR.render();
    let good = GOOD.render();
    let info = INFO.render();
    let reset = anstyle::Reset.render();

    let start = String::from_utf8_lossy(&msg.buffer[0..msg.byte_offset]);
    let column_number =
        unicode_segmentation::UnicodeSegmentation::graphemes(start.as_ref(), true).count() + 1;
    match &msg.corrections {
        typos::Status::Valid => {}
        typos::Status::Invalid => {
            let divider = ":";
            writeln!(
                stdout().lock(),
                "{info}{}{divider}{column_number}{reset}: `{error}{}{reset}` is disallowed",
                context_display(&msg.context),
                msg.typo,
            )?;
        }
        typos::Status::Corrections(corrections) => {
            let divider = ":";
            writeln!(
                stdout().lock(),
                "{info}{}{divider}{column_number}{reset}: `{error}{}{reset}` -> {}",
                context_display(&msg.context),
                msg.typo,
                itertools::join(
                    corrections.iter().map(|s| format!("`{good}{s}{reset}`")),
                    ", "
                )
            )?;
        }
    }

    Ok(())
}

fn print_long_correction(msg: &Typo<'_>) -> Result<(), std::io::Error> {
    let error = ERROR.render();
    let good = GOOD.render();
    let info = INFO.render();
    let reset = anstyle::Reset.render();

    let stdout = stdout();
    let mut handle = stdout.lock();

    let line = String::from_utf8_lossy(msg.buffer.as_ref());
    let line = line.replace('\t', " ");
    let start = String::from_utf8_lossy(&msg.buffer[0..msg.byte_offset]);
    let column_number =
        unicode_segmentation::UnicodeSegmentation::graphemes(start.as_ref(), true).count() + 1;
    match &msg.corrections {
        typos::Status::Valid => {}
        typos::Status::Invalid => {
            writeln!(
                handle,
                "{error}error{reset}: `{error}{}{reset}` is disallowed",
                msg.typo,
            )?;
        }
        typos::Status::Corrections(corrections) => {
            writeln!(
                handle,
                "{error}error{reset}: `{error}{}{reset}` should be {}",
                msg.typo,
                itertools::join(
                    corrections.iter().map(|s| format!("`{good}{s}{reset}`")),
                    ", "
                )
            )?;
        }
    }
    let divider = ":";
    writeln!(
        handle,
        "{info}  --> {reset}{}{divider}{column_number}",
        context_display(&msg.context),
    )?;

    if let Some(Context::File(context)) = &msg.context {
        let line_num = context.line_num.to_string();
        let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();
        let line = line.trim_end();

        let visible_column = calculate_visible_column_width(start.as_ref());
        let visible_len = calculate_visible_column_width(msg.typo);

        let hl_indent: String = itertools::repeat_n(" ", visible_column).collect();
        let hl: String = itertools::repeat_n("^", visible_len).collect();

        writeln!(handle, "{info}{line_indent} |{reset}")?;
        writeln!(handle, "{info}{line_num} |{reset} {line}")?;
        writeln!(
            handle,
            "{info}{line_indent} |{reset} {hl_indent}{error}{hl}{reset}",
        )?;
        writeln!(handle, "{info}{line_indent} |{reset}")?;
    }

    Ok(())
}

fn calculate_visible_column_width(str: &str) -> usize {
    let mut result = 0;
    let graphemes = unicode_segmentation::UnicodeSegmentation::graphemes(str, true);
    for grapheme in graphemes {
        result += if grapheme == "\t" {
            // TODO: config tab width
            1
        } else if is_emoji(grapheme) {
            // UnicodeWidthStr::width doesn't cover for emoji according to their README.
            // See: https://github.com/unicode-rs/unicode-width#unicode-width
            // Also, the actual rendered column width may differ from calculation, especially for emojis.
            // In here, we expect emoji renderers should render this emoji properly.
            2
        } else {
            UnicodeWidthStr::width(grapheme)
        }
    }

    result
}

fn is_emoji(grapheme: &str) -> bool {
    if grapheme.is_ascii() {
        return false;
    }

    for ch in grapheme.chars() {
        if unic_emoji_char::is_emoji(ch) {
            return true;
        }
    }

    false
}

fn context_display<'c>(context: &'c Option<Context<'c>>) -> &'c dyn std::fmt::Display {
    context
        .as_ref()
        .map(|c| c as &dyn std::fmt::Display)
        .unwrap_or(&"")
}

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
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("failed to generate SARIF output: {error}"),
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_visible_column_width_visible_ascii() {
        for c in '!'..'~' {
            assert_eq!(1, calculate_visible_column_width(&c.to_string()));
        }
    }

    #[test]
    fn test_calculate_visible_column_width_horizontal_tab() {
        assert_eq!(1, calculate_visible_column_width("\t"));
    }

    #[test]
    fn test_calculate_visible_column_width_latin_cyrillic() {
        let latin_cyrillic_chars = [
            "À", /* U+00C0; Latin Capital Letter A with Grave */
            "À", /* U+0041 U+0300; Latin Capital Letter A, Combining Grave Accent */
            "А", /* U+0410 Cyrillic Capital Letter A */
        ];
        for (i, ch) in latin_cyrillic_chars.iter().enumerate() {
            let width = calculate_visible_column_width(ch);
            assert_eq!(1, width, "latin_cyrillic[{i}]: {ch}",);
        }
    }

    #[test]
    fn test_calculate_visible_column_width_cjk() {
        let cjk_chars = [
            "中", /* U+4E2D */
            "あ", /* U+3042 */
            "한", /* U+1F635 U+200D U+1F4AB, NFC Korean */
            "한", /* U+1F441 U+FE0F U+200D U+1F5E8 U+FE0F, NFD Korean */
        ];
        for (i, ch) in cjk_chars.iter().enumerate() {
            let width = calculate_visible_column_width(ch);
            assert_eq!(2, width, "cjk[{i}]: {ch}");
        }
    }

    #[test]
    fn test_calculate_visible_column_width_simple_emojis() {
        // First non-component emojis of each groups in "Full Emoji List, v14.0"
        // https://unicode.org/Public/emoji/14.0/emoji-test.txt
        let simple_emojis = [
            "😀", /* U+1F600 */
            "👋", /* U+1F44B */
            "🐵", /* U+1F435 */
            "🍇", /* U+1F347 */
            "🌍", /* U+1F30D */
            "🎃", /* U+1F383 */
            "👓", /* U+1F453 */
            "🏧", /* U+1F3E7 */
            "🏁", /* U+1F3C1 */
        ];
        for (i, ch) in simple_emojis.iter().enumerate() {
            let width = calculate_visible_column_width(ch);
            assert_eq!(2, width, "emoji[{i}]: {ch}");
        }
    }

    #[test]
    fn test_calculate_visible_column_width_zwj_sequences() {
        let zwj_sequences = [
            "😵‍💫", /* U+1F635 U+200D U+1F4AB */
            "👁️‍🗨️", /* U+1F441 U+FE0F U+200D U+1F5E8 U+FE0F */
        ];
        for (i, ch) in zwj_sequences.iter().enumerate() {
            let width = calculate_visible_column_width(ch);
            assert_eq!(2, width, "zwj[{i}]: {ch}");
        }
    }
}
