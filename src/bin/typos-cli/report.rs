#![allow(clippy::needless_update)]

use std::io::{self, Write};
use std::sync::atomic;

use typos_cli::report::{Context, Message, Report, Typo};

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
        let _ = self.typos_found.compare_exchange(
            false,
            msg.is_correction(),
            atomic::Ordering::Relaxed,
            atomic::Ordering::Relaxed,
        );
        let _ = self
            .errors_found
            .compare_exchange(
                false,
                msg.is_error(),
                atomic::Ordering::Relaxed,
                atomic::Ordering::Relaxed,
            )
            .unwrap();
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

#[derive(Copy, Clone, Debug)]
pub struct PrintBrief;

impl Report for PrintBrief {
    fn report(&self, msg: Message) -> Result<(), std::io::Error> {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{}", msg);
            }
            Message::Typo(msg) => print_brief_correction(msg)?,
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

#[derive(Copy, Clone, Debug)]
pub struct PrintLong;

impl Report for PrintLong {
    fn report(&self, msg: Message) -> Result<(), std::io::Error> {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{}", msg);
            }
            Message::Typo(msg) => print_long_correction(msg)?,
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

fn print_brief_correction(msg: &Typo) -> Result<(), std::io::Error> {
    let line = String::from_utf8_lossy(msg.buffer.as_ref());
    let line = line.replace("\t", " ");
    let column = unicode_segmentation::UnicodeSegmentation::graphemes(
        line.get(0..msg.byte_offset).unwrap(),
        true,
    )
    .count();
    match &msg.corrections {
        typos::Status::Valid => {}
        typos::Status::Invalid => {
            writeln!(
                io::stdout(),
                "{}:{}: `{}` is disallowed",
                context_display(&msg.context),
                column,
                msg.typo,
            )?;
        }
        typos::Status::Corrections(corrections) => {
            writeln!(
                io::stdout(),
                "{}:{}: `{}` -> {}",
                context_display(&msg.context),
                column,
                msg.typo,
                itertools::join(corrections.iter().map(|s| format!("`{}`", s)), ", ")
            )?;
        }
    }

    Ok(())
}

fn print_long_correction(msg: &Typo) -> Result<(), std::io::Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let line = String::from_utf8_lossy(msg.buffer.as_ref());
    let line = line.replace("\t", " ");
    let column = unicode_segmentation::UnicodeSegmentation::graphemes(
        line.get(0..msg.byte_offset).unwrap(),
        true,
    )
    .count();
    match &msg.corrections {
        typos::Status::Valid => {}
        typos::Status::Invalid => {
            writeln!(handle, "error: `{}` is disallowed`", msg.typo,)?;
        }
        typos::Status::Corrections(corrections) => {
            writeln!(
                handle,
                "error: `{}` should be {}",
                msg.typo,
                itertools::join(corrections.iter().map(|s| format!("`{}`", s)), ", ")
            )?;
        }
    }
    writeln!(handle, "  --> {}:{}", context_display(&msg.context), column)?;

    if let Some(Context::File(context)) = &msg.context {
        let line_num = context.line_num.to_string();
        let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

        let hl_indent: String = itertools::repeat_n(" ", column).collect();
        let hl: String = itertools::repeat_n("^", msg.typo.len()).collect();

        writeln!(handle, "{} |", line_indent)?;
        writeln!(handle, "{} | {}", line_num, line.trim_end())?;
        writeln!(handle, "{} | {}{}", line_indent, hl_indent, hl)?;
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
