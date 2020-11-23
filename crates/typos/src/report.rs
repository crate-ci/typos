#![allow(clippy::needless_update)]

use std::borrow::Cow;
use std::io::{self, Write};
use std::sync::atomic;

#[derive(Clone, Debug, serde::Serialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Message<'m> {
    BinaryFile(BinaryFile<'m>),
    Typo(Typo<'m>),
    File(File<'m>),
    Parse(Parse<'m>),
    Error(Error<'m>),
}

impl<'m> Message<'m> {
    pub fn is_correction(&self) -> bool {
        match self {
            Message::BinaryFile(_) => false,
            Message::Typo(c) => c.corrections.is_correction(),
            Message::File(_) => false,
            Message::Parse(_) => false,
            Message::Error(_) => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Message::BinaryFile(_) => false,
            Message::Typo(_) => false,
            Message::File(_) => false,
            Message::Parse(_) => false,
            Message::Error(_) => true,
        }
    }

    pub fn context(self, context: Option<Context<'m>>) -> Self {
        match self {
            Message::Typo(typo) => {
                let typo = typo.context(context);
                Message::Typo(typo)
            }
            Message::Parse(parse) => {
                let parse = parse.context(context);
                Message::Parse(parse)
            }
            Message::Error(error) => {
                let error = error.context(context);
                Message::Error(error)
            }
            _ => self,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_more::Display, derive_setters::Setters)]
#[display(fmt = "Skipping binary file {}", "path.display()")]
#[non_exhaustive]
pub struct BinaryFile<'m> {
    pub path: &'m std::path::Path,
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct Typo<'m> {
    #[serde(flatten)]
    pub context: Option<Context<'m>>,
    #[serde(skip)]
    pub buffer: Cow<'m, [u8]>,
    pub byte_offset: usize,
    pub typo: &'m str,
    pub corrections: crate::Status<'m>,
}

impl<'m> Default for Typo<'m> {
    fn default() -> Self {
        Self {
            context: None,
            buffer: Cow::Borrowed(&[]),
            byte_offset: 0,
            typo: "",
            corrections: crate::Status::Invalid,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_more::From)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Context<'m> {
    File(FileContext<'m>),
    Path(PathContext<'m>),
}

impl<'m> std::fmt::Display for Context<'m> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Context::File(c) => write!(f, "{}:{}", c.path.display(), c.line_num),
            Context::Path(c) => write!(f, "{}", c.path.display()),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct FileContext<'m> {
    pub path: &'m std::path::Path,
    pub line_num: usize,
}

impl<'m> Default for FileContext<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
            line_num: 0,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct PathContext<'m> {
    pub path: &'m std::path::Path,
}

impl<'m> Default for PathContext<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ParseKind {
    Identifier,
    Word,
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct File<'m> {
    pub path: &'m std::path::Path,
}

impl<'m> File<'m> {
    pub fn new(path: &'m std::path::Path) -> Self {
        Self { path }
    }
}

impl<'m> Default for File<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct Parse<'m> {
    #[serde(flatten)]
    pub context: Option<Context<'m>>,
    pub kind: ParseKind,
    pub data: Vec<&'m str>,
}

impl<'m> Default for Parse<'m> {
    fn default() -> Self {
        Self {
            context: None,
            kind: ParseKind::Identifier,
            data: vec![],
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct Error<'m> {
    #[serde(flatten)]
    pub context: Option<Context<'m>>,
    pub msg: String,
}

impl<'m> Error<'m> {
    pub fn new(msg: String) -> Self {
        Self { context: None, msg }
    }
}

impl<'m> Default for Error<'m> {
    fn default() -> Self {
        Self {
            context: None,
            msg: "".to_owned(),
        }
    }
}

pub trait Report: Send + Sync {
    fn report(&self, msg: Message) -> Result<(), std::io::Error>;
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
        self.typos_found
            .compare_and_swap(false, msg.is_correction(), atomic::Ordering::Relaxed);
        self.errors_found
            .compare_and_swap(false, msg.is_error(), atomic::Ordering::Relaxed);
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
                writeln!(io::stdout(), "{}", itertools::join(msg.data.iter(), " "))?;
            }
            Message::Error(msg) => {
                log::error!("{}: {}", context_display(&msg.context), msg.msg);
            }
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
                writeln!(io::stdout(), "{}", itertools::join(msg.data.iter(), " "))?;
            }
            Message::Error(msg) => {
                log::error!("{}: {}", context_display(&msg.context), msg.msg);
            }
        }
        Ok(())
    }
}

fn print_brief_correction(msg: &Typo) -> Result<(), std::io::Error> {
    match &msg.corrections {
        crate::Status::Valid => {}
        crate::Status::Invalid => {
            writeln!(
                io::stdout(),
                "{}:{}: {} is disallowed",
                context_display(&msg.context),
                msg.byte_offset,
                msg.typo,
            )?;
        }
        crate::Status::Corrections(corrections) => {
            writeln!(
                io::stdout(),
                "{}:{}: {} -> {}",
                context_display(&msg.context),
                msg.byte_offset,
                msg.typo,
                itertools::join(corrections.iter(), ", ")
            )?;
        }
    }

    Ok(())
}

fn print_long_correction(msg: &Typo) -> Result<(), std::io::Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    match &msg.corrections {
        crate::Status::Valid => {}
        crate::Status::Invalid => {
            writeln!(
                handle,
                "{}:{}: {} is disallowed",
                context_display(&msg.context),
                msg.byte_offset,
                msg.typo,
            )?;
        }
        crate::Status::Corrections(corrections) => {
            writeln!(
                handle,
                "error: `{}` should be {}",
                msg.typo,
                itertools::join(corrections.iter(), ", ")
            )?;
        }
    }
    writeln!(
        handle,
        "  --> {}:{}",
        context_display(&msg.context),
        msg.byte_offset
    )?;

    if let Some(Context::File(context)) = &msg.context {
        let line_num = context.line_num.to_string();
        let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

        let hl_indent: String = itertools::repeat_n(" ", msg.byte_offset).collect();
        let hl: String = itertools::repeat_n("^", msg.typo.len()).collect();

        let line = String::from_utf8_lossy(msg.buffer.as_ref());
        let line = line.replace("\t", " ");
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
