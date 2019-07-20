use std::borrow::Cow;
use std::io::{self, Write};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Message<'m> {
    BinaryFile(BinaryFile<'m>),
    Correction(Correction<'m>),
    FilenameCorrection(FilenameCorrection<'m>),
}

impl<'m> From<BinaryFile<'m>> for Message<'m> {
    fn from(msg: BinaryFile<'m>) -> Self {
        Message::BinaryFile(msg)
    }
}

impl<'m> From<Correction<'m>> for Message<'m> {
    fn from(msg: Correction<'m>) -> Self {
        Message::Correction(msg)
    }
}

impl<'m> From<FilenameCorrection<'m>> for Message<'m> {
    fn from(msg: FilenameCorrection<'m>) -> Self {
        Message::FilenameCorrection(msg)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BinaryFile<'m> {
    pub path: &'m std::path::Path,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub struct FilenameCorrection<'m> {
    pub path: &'m std::path::Path,
    pub typo: &'m str,
    pub correction: Cow<'m, str>,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

pub type Report = fn(msg: Message);

pub fn print_silent(_: Message) {}

pub fn print_brief(msg: Message) {
    match msg {
        Message::BinaryFile(msg) => {
            println!("Skipping binary file {}", msg.path.display(),);
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
    }
}

pub fn print_long(msg: Message) {
    match msg {
        Message::BinaryFile(msg) => {
            println!("Skipping binary file {}", msg.path.display(),);
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

pub fn print_json(msg: Message) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}
