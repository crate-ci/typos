use std::borrow::Cow;
use std::io::{self, Write};

#[derive(Clone, Debug, serde::Serialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Message<'m> {
    BinaryFile(BinaryFile<'m>),
    Correction(Correction<'m>),
    FilenameCorrection(FilenameCorrection<'m>),
    File(File<'m>),
    Parse(Parse<'m>),
    PathError(PathError<'m>),
    Error(Error),
    #[serde(skip)]
    __NonExhaustive,
}

#[derive(Clone, Debug, serde::Serialize, derive_more::Display)]
#[display(fmt = "Skipping binary file {}", "path.display()")]
pub struct BinaryFile<'m> {
    pub path: &'m std::path::Path,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Correction<'m> {
    pub path: &'m std::path::Path,
    #[serde(skip)]
    pub line: &'m [u8],
    pub line_num: usize,
    pub col_num: usize,
    pub typo: &'m str,
    pub correction: Cow<'m, str>,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FilenameCorrection<'m> {
    pub path: &'m std::path::Path,
    pub typo: &'m str,
    pub correction: Cow<'m, str>,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub enum ParseKind {
    Identifier,
    Word,
    #[doc(hidden)]
    __NonExhaustive,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct File<'m> {
    pub path: &'m std::path::Path,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

impl<'m> File<'m> {
    pub fn new(path: &'m std::path::Path) -> Self {
        Self {
            path,
            non_exhaustive: (),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Parse<'m> {
    pub path: &'m std::path::Path,
    pub kind: ParseKind,
    pub data: Vec<&'m str>,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PathError<'m> {
    pub path: &'m std::path::Path,
    pub msg: String,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Error {
    pub msg: String,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self {
            msg,
            non_exhaustive: (),
        }
    }
}

pub trait Report: Send + Sync {
    fn report(&self, msg: Message);
}

pub struct PrintSilent;

impl Report for PrintSilent {
    fn report(&self, _msg: Message) {}
}

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
                    msg.col_num,
                    msg.typo,
                    msg.correction
                );
            }
            Message::FilenameCorrection(msg) => {
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
            Message::__NonExhaustive => {
                unreachable!("Non-creatable case");
            }
        }
    }
}

pub struct PrintLong;

impl Report for PrintLong {
    fn report(&self, msg: Message) {
        match msg {
            Message::BinaryFile(msg) => {
                println!("{}", msg);
            }
            Message::Correction(msg) => print_long_correction(msg),
            Message::FilenameCorrection(msg) => {
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
            Message::__NonExhaustive => {
                unreachable!("Non-creatable case");
            }
        }
    }
}

fn print_long_correction(msg: Correction) {
    let line_num = msg.line_num.to_string();
    let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

    let hl_indent: String = itertools::repeat_n(" ", msg.col_num).collect();
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
        msg.col_num
    )
    .unwrap();
    writeln!(handle, "{} |", line_indent).unwrap();
    writeln!(handle, "{} | {}", msg.line_num, line.trim_end()).unwrap();
    writeln!(handle, "{} | {}{}", line_indent, hl_indent, hl).unwrap();
    writeln!(handle, "{} |", line_indent).unwrap();
}

pub struct PrintJson;

impl Report for PrintJson {
    fn report(&self, msg: Message) {
        println!("{}", serde_json::to_string(&msg).unwrap());
    }
}
