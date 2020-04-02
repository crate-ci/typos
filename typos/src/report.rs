use std::borrow::Cow;
use std::io::{self, Write};

#[derive(Clone, Debug, serde::Serialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Message<'m> {
    BinaryFile(BinaryFile<'m>),
    Correction(Correction<'m>),
    PathCorrection(PathCorrection<'m>),
    File(File<'m>),
    Parse(Parse<'m>),
    PathError(PathError<'m>),
    Error(Error),
}

#[derive(Clone, Debug, serde::Serialize, derive_more::Display, derive_setters::Setters)]
#[display(fmt = "Skipping binary file {}", "path.display()")]
#[non_exhaustive]
pub struct BinaryFile<'m> {
    pub path: &'m std::path::Path,
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct Correction<'m> {
    pub path: &'m std::path::Path,
    #[serde(skip)]
    pub line: &'m [u8],
    pub line_num: usize,
    pub byte_offset: usize,
    pub typo: &'m str,
    pub correction: Cow<'m, str>,
}

impl<'m> Default for Correction<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
            line: b"",
            line_num: 0,
            byte_offset: 0,
            typo: "",
            correction: Cow::Borrowed(""),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_setters::Setters)]
#[non_exhaustive]
pub struct PathCorrection<'m> {
    pub path: &'m std::path::Path,
    pub byte_offset: usize,
    pub typo: &'m str,
    pub correction: Cow<'m, str>,
}

impl<'m> Default for PathCorrection<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
            byte_offset: 0,
            typo: "",
            correction: Cow::Borrowed(""),
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub path: &'m std::path::Path,
    pub kind: ParseKind,
    pub data: Vec<&'m str>,
}

impl<'m> Default for Parse<'m> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
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
    fn report(&self, msg: Message);
}

#[derive(Copy, Clone, Debug)]
pub struct PrintSilent;

impl Report for PrintSilent {
    fn report(&self, _msg: Message) {}
}

#[derive(Copy, Clone, Debug)]
pub struct PrintBrief;

impl Report for PrintBrief {
    fn report(&self, msg: Message) {
        match msg {
            Message::BinaryFile(msg) => {
                println!("{}", msg);
            }
            Message::Correction(msg) => {
                println!(
                    "{}:{}:{}: {} -> {}",
                    msg.path.display(),
                    msg.line_num,
                    msg.byte_offset,
                    msg.typo,
                    msg.correction
                );
            }
            Message::PathCorrection(msg) => {
                println!("{}: {} -> {}", msg.path.display(), msg.typo, msg.correction);
            }
            Message::File(msg) => {
                println!("{}", msg.path.display());
            }
            Message::Parse(msg) => {
                println!("{}", itertools::join(msg.data.iter(), " "));
            }
            Message::PathError(msg) => {
                println!("{}: {}", msg.path.display(), msg.msg);
            }
            Message::Error(msg) => {
                println!("{}", msg.msg);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PrintLong;

impl Report for PrintLong {
    fn report(&self, msg: Message) {
        match msg {
            Message::BinaryFile(msg) => {
                println!("{}", msg);
            }
            Message::Correction(msg) => print_long_correction(msg),
            Message::PathCorrection(msg) => {
                println!(
                    "{}: error: `{}` should be `{}`",
                    msg.path.display(),
                    msg.typo,
                    msg.correction
                );
            }
            Message::File(msg) => {
                println!("{}", msg.path.display());
            }
            Message::Parse(msg) => {
                println!("{}", itertools::join(msg.data.iter(), " "));
            }
            Message::PathError(msg) => {
                println!("{}: {}", msg.path.display(), msg.msg);
            }
            Message::Error(msg) => {
                println!("{}", msg.msg);
            }
        }
    }
}

fn print_long_correction(msg: Correction) {
    let line_num = msg.line_num.to_string();
    let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

    let hl_indent: String = itertools::repeat_n(" ", msg.byte_offset).collect();
    let hl: String = itertools::repeat_n("^", msg.typo.len()).collect();

    let line = String::from_utf8_lossy(msg.line);
    let line = line.replace("\t", " ");

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(
        handle,
        "error: `{}` should be `{}`",
        msg.typo, msg.correction
    )
    .unwrap();
    writeln!(
        handle,
        "  --> {}:{}:{}",
        msg.path.display(),
        msg.line_num,
        msg.byte_offset
    )
    .unwrap();
    writeln!(handle, "{} |", line_indent).unwrap();
    writeln!(handle, "{} | {}", msg.line_num, line.trim_end()).unwrap();
    writeln!(handle, "{} | {}{}", line_indent, hl_indent, hl).unwrap();
    writeln!(handle, "{} |", line_indent).unwrap();
}

#[derive(Copy, Clone, Debug)]
pub struct PrintJson;

impl Report for PrintJson {
    fn report(&self, msg: Message) {
        println!("{}", serde_json::to_string(&msg).unwrap());
    }
}
