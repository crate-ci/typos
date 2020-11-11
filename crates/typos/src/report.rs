#![allow(clippy::needless_update)]

use std::borrow::Cow;
use std::io::{self, Write};

#[derive(Clone, Debug, serde::Serialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Message<'m> {
    BinaryFile(BinaryFile<'m>),
    Typo(Typo<'m>),
    File(File<'m>),
    Parse(Parse<'m>),
    PathError(PathError<'m>),
    Error(Error),
}

impl<'m> Message<'m> {
    pub fn is_correction(&self) -> bool {
        match self {
            Message::BinaryFile(_) => false,
            Message::Typo(c) => c.corrections.is_correction(),
            Message::File(_) => false,
            Message::Parse(_) => false,
            Message::PathError(_) => false,
            Message::Error(_) => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Message::BinaryFile(_) => false,
            Message::Typo(_) => false,
            Message::File(_) => false,
            Message::Parse(_) => false,
            Message::PathError(_) => true,
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
pub struct PathError<'m> {
    pub path: &'m std::path::Path,
    pub msg: String,
}

impl<'m> Default for PathError<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
            msg: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Default for Error {
    fn default() -> Self {
        Self { msg: "".to_owned() }
    }
}

pub trait Report: Send + Sync {
    fn report(&self, msg: Message) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub struct PrintSilent;

impl Report for PrintSilent {
    fn report(&self, msg: Message) -> bool {
        msg.is_correction()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PrintBrief;

impl Report for PrintBrief {
    fn report(&self, msg: Message) -> bool {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{}", msg);
            }
            Message::Typo(msg) => print_brief_correction(msg),
            Message::File(msg) => {
                println!("{}", msg.path.display());
            }
            Message::Parse(msg) => {
                println!("{}", itertools::join(msg.data.iter(), " "));
            }
            Message::PathError(msg) => {
                log::error!("{}: {}", msg.path.display(), msg.msg);
            }
            Message::Error(msg) => {
                log::error!("{}", msg.msg);
            }
        }
        msg.is_correction()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PrintLong;

impl Report for PrintLong {
    fn report(&self, msg: Message) -> bool {
        match &msg {
            Message::BinaryFile(msg) => {
                log::info!("{}", msg);
            }
            Message::Typo(msg) => print_long_correction(msg),
            Message::File(msg) => {
                println!("{}", msg.path.display());
            }
            Message::Parse(msg) => {
                println!("{}", itertools::join(msg.data.iter(), " "));
            }
            Message::PathError(msg) => {
                log::error!("{}: {}", msg.path.display(), msg.msg);
            }
            Message::Error(msg) => {
                log::error!("{}", msg.msg);
            }
        }
        msg.is_correction()
    }
}

fn print_brief_correction(msg: &Typo) {
    match &msg.corrections {
        crate::Status::Valid => {}
        crate::Status::Invalid => {
            println!(
                "{}:{}: {} is disallowed",
                context_display(&msg.context),
                msg.byte_offset,
                msg.typo,
            );
        }
        crate::Status::Corrections(corrections) => {
            println!(
                "{}:{}: {} -> {}",
                context_display(&msg.context),
                msg.byte_offset,
                msg.typo,
                itertools::join(corrections.iter(), ", ")
            );
        }
    }
}

fn print_long_correction(msg: &Typo) {
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
            )
            .unwrap();
        }
        crate::Status::Corrections(corrections) => {
            writeln!(
                handle,
                "error: `{}` should be {}",
                msg.typo,
                itertools::join(corrections.iter(), ", ")
            )
            .unwrap();
        }
    }
    writeln!(
        handle,
        "  --> {}:{}",
        context_display(&msg.context),
        msg.byte_offset
    )
    .unwrap();

    if let Some(Context::File(context)) = &msg.context {
        let line_num = context.line_num.to_string();
        let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

        let hl_indent: String = itertools::repeat_n(" ", msg.byte_offset).collect();
        let hl: String = itertools::repeat_n("^", msg.typo.len()).collect();

        let line = String::from_utf8_lossy(msg.buffer.as_ref());
        let line = line.replace("\t", " ");
        writeln!(handle, "{} |", line_indent).unwrap();
        writeln!(handle, "{} | {}", line_num, line.trim_end()).unwrap();
        writeln!(handle, "{} | {}{}", line_indent, hl_indent, hl).unwrap();
        writeln!(handle, "{} |", line_indent).unwrap();
    }
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
    fn report(&self, msg: Message) -> bool {
        println!("{}", serde_json::to_string(&msg).unwrap());
        msg.is_correction()
    }
}
