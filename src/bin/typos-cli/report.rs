#![allow(clippy::needless_update)]

use std::io::{self, Write};
use std::sync::atomic;

use unicode_width::UnicodeWidthStr;

use typos_cli::report::{Context, Message, Report, Typo};

#[derive(Copy, Clone, Debug)]
pub struct Palette {
    error: yansi::Style,
    info: yansi::Style,
    strong: yansi::Style,
}

impl Palette {
    pub fn colored() -> Self {
        Self {
            error: yansi::Style::new(yansi::Color::Red),
            info: yansi::Style::new(yansi::Color::Blue),
            strong: yansi::Style::default().bold(),
        }
    }

    pub fn plain() -> Self {
        Self {
            error: yansi::Style::default(),
            info: yansi::Style::default(),
            strong: yansi::Style::default(),
        }
    }
}

pub struct MessageStatus<'r> {
    typos_found: atomic::AtomicBool,
    errors_found: atomic::AtomicBool,
    reporter: &'r dyn Report,
}

impl<'r> MessageStatus<'r> {
    pub fn new(reporter: &'r dyn Report) -> Self {
        Self {
            typos_found: atomic::AtomicBool::new(false),
            errors_found: atomic::AtomicBool::new(false),
            reporter,
        }
    }

    pub fn typos_found(&self) -> bool {
        self.typos_found.load(atomic::Ordering::Relaxed)
    }

    pub fn errors_found(&self) -> bool {
        self.errors_found.load(atomic::Ordering::Relaxed)
    }
}

impl<'r> Report for MessageStatus<'r> {
    fn report(&self, msg: Message) -> Result<(), std::io::Error> {
        if msg.is_correction() {
            self.typos_found.store(true, atomic::Ordering::Relaxed);
        }
        if msg.is_error() {
            self.errors_found.store(true, atomic::Ordering::Relaxed);
        }
        self.reporter.report(msg)
    }
}

#[derive(Debug, Default)]
pub struct PrintSilent;

impl Report for PrintSilent {
    fn report(&self, _msg: Message) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub struct PrintBrief {
    pub stdout_palette: Palette,
    pub stderr_palette: Palette,
}

impl Report for PrintBrief {
    fn report(&self, msg: Message) -> Result<(), std::io::Error> {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{}", msg);
            }
            Message::Typo(msg) => print_brief_correction(msg, self.stdout_palette)?,
            Message::File(msg) => {
                writeln!(io::stdout(), "{}", msg.path.display())?;
            }
            Message::Parse(msg) => {
                writeln!(io::stdout(), "{}", msg.data)?;
            }
            Message::Error(msg) => {
                log::error!("{}: {}", context_display(&msg.context), msg.msg);
            }
            _ => unimplemented!("New message {:?}", msg),
        }
        Ok(())
    }
}

pub struct PrintLong {
    pub stdout_palette: Palette,
    pub stderr_palette: Palette,
}

impl Report for PrintLong {
    fn report(&self, msg: Message) -> Result<(), std::io::Error> {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{}", msg);
            }
            Message::Typo(msg) => print_long_correction(msg, self.stdout_palette)?,
            Message::File(msg) => {
                writeln!(io::stdout(), "{}", msg.path.display())?;
            }
            Message::Parse(msg) => {
                writeln!(io::stdout(), "{}", msg.data)?;
            }
            Message::Error(msg) => {
                log::error!("{}: {}", context_display(&msg.context), msg.msg);
            }
            _ => unimplemented!("New message {:?}", msg),
        }
        Ok(())
    }
}

fn print_brief_correction(msg: &Typo, palette: Palette) -> Result<(), std::io::Error> {
    let line = String::from_utf8_lossy(msg.buffer.as_ref());
    let line = line.replace('\t', " ");
    let column = unicode_segmentation::UnicodeSegmentation::graphemes(
        line.get(0..msg.byte_offset).unwrap(),
        true,
    )
    .count();
    match &msg.corrections {
        typos::Status::Valid => {}
        typos::Status::Invalid => {
            let divider = ":";
            writeln!(
                io::stdout(),
                "{}{}{}: {}",
                palette.info.paint(context_display(&msg.context)),
                palette.info.paint(divider),
                palette.info.paint(column),
                palette
                    .strong
                    .paint(format_args!("`{}` is disallowed:", msg.typo)),
            )?;
        }
        typos::Status::Corrections(corrections) => {
            let divider = ":";
            writeln!(
                io::stdout(),
                "{}{}{}: {}",
                palette.info.paint(context_display(&msg.context)),
                palette.info.paint(divider),
                palette.info.paint(column),
                palette.strong.paint(format_args!(
                    "`{}` -> {}",
                    msg.typo,
                    itertools::join(corrections.iter().map(|s| format!("`{}`", s)), ", ")
                )),
            )?;
        }
    }

    Ok(())
}

fn print_long_correction(msg: &Typo, palette: Palette) -> Result<(), std::io::Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let line = String::from_utf8_lossy(msg.buffer.as_ref());
    let line = line.replace('\t', " ");
    let start = String::from_utf8_lossy(&msg.buffer[0..msg.byte_offset]);
    let column = unicode_segmentation::UnicodeSegmentation::graphemes(start.as_ref(), true).count();
    match &msg.corrections {
        typos::Status::Valid => {}
        typos::Status::Invalid => {
            writeln!(
                handle,
                "{}: {}",
                palette.error.paint("error"),
                palette
                    .strong
                    .paint(format_args!("`{}` is disallowed`", msg.typo))
            )?;
        }
        typos::Status::Corrections(corrections) => {
            writeln!(
                handle,
                "{}: {}",
                palette.error.paint("error"),
                palette.strong.paint(format_args!(
                    "`{}` should be {}",
                    msg.typo,
                    itertools::join(corrections.iter().map(|s| format!("`{}`", s)), ", ")
                ))
            )?;
        }
    }
    let divider = ":";
    writeln!(
        handle,
        "  --> {}{}{}",
        palette.info.paint(context_display(&msg.context)),
        palette.info.paint(divider),
        palette.info.paint(column)
    )?;

    if let Some(Context::File(context)) = &msg.context {
        let line_num = context.line_num.to_string();
        let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

        let visible_column = UnicodeWidthStr::width(start.as_ref());
        let visible_len = UnicodeWidthStr::width(msg.typo);

        let hl_indent: String = itertools::repeat_n(" ", visible_column).collect();
        let hl: String = itertools::repeat_n("^", visible_len).collect();

        writeln!(handle, "{} |", line_indent)?;
        writeln!(
            handle,
            "{} | {}",
            palette.info.paint(line_num),
            line.trim_end()
        )?;
        writeln!(
            handle,
            "{} | {}{}",
            line_indent,
            hl_indent,
            palette.error.paint(hl)
        )?;
        writeln!(handle, "{} |", line_indent)?;
    }

    Ok(())
}

fn context_display<'c>(context: &'c Option<Context<'c>>) -> &'c dyn std::fmt::Display {
    context
        .as_ref()
        .map(|c| c as &dyn std::fmt::Display)
        .unwrap_or(&"")
}

#[derive(Copy, Clone, Debug)]
pub struct PrintJson;

impl Report for PrintJson {
    fn report(&self, msg: Message) -> Result<(), std::io::Error> {
        writeln!(io::stdout(), "{}", serde_json::to_string(&msg).unwrap())?;
        Ok(())
    }
}
