#![allow(clippy::needless_update)]

use std::borrow::Cow;

pub trait Report: Send + Sync {
    fn report(&self, msg: Message<'_>) -> Result<(), std::io::Error>;

    fn generate_final_result(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[derive(Clone, Debug, serde::Serialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Message<'m> {
    BinaryFile(BinaryFile<'m>),
    Typo(Typo<'m>),
    FileType(FileType<'m>),
    File(File<'m>),
    Parse(Parse<'m>),
    Error(Error<'m>),
}

impl<'m> Message<'m> {
    pub fn is_typo(&self) -> bool {
        match self {
            Message::BinaryFile(_) => false,
            Message::Typo(c) => !c.corrections.is_valid(),
            Message::FileType(_) => false,
            Message::File(_) => false,
            Message::Parse(_) => false,
            Message::Error(_) => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            Message::BinaryFile(_) => false,
            Message::Typo(_) => false,
            Message::FileType(_) => false,
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
#[display("Skipping binary file {}", path.display())]
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
    pub corrections: typos::Status<'m>,
}

impl Default for Typo<'_> {
    fn default() -> Self {
        Self {
            context: None,
            buffer: Cow::Borrowed(&[]),
            byte_offset: 0,
            typo: "",
            corrections: typos::Status::Invalid,
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

impl std::fmt::Display for Context<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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

impl Default for FileContext<'_> {
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

impl Default for PathContext<'_> {
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
pub struct FileType<'m> {
    pub path: &'m std::path::Path,
    pub file_type: Option<&'m str>,
}

impl<'m> FileType<'m> {
    pub fn new(path: &'m std::path::Path, file_type: Option<&'m str>) -> Self {
        Self { path, file_type }
    }
}

impl Default for FileType<'_> {
    fn default() -> Self {
        Self {
            path: std::path::Path::new("-"),
            file_type: None,
        }
    }
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

impl Default for File<'_> {
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
    pub data: &'m str,
}

impl Default for Parse<'_> {
    fn default() -> Self {
        Self {
            context: None,
            kind: ParseKind::Identifier,
            data: "",
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

impl Error<'_> {
    pub fn new(msg: String) -> Self {
        Self { context: None, msg }
    }
}

impl Default for Error<'_> {
    fn default() -> Self {
        Self {
            context: None,
            msg: "".to_owned(),
        }
    }
}
