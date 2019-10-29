#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum ErrorKind {
    #[display(fmt = "Invalid word")]
    InvalidWord,
    #[display(fmt = "IO Error")]
    IoError,
}

impl ErrorKind {
    pub fn into_error(self) -> Error {
        Error {
            kind: self,
            msg: None,
            source: None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub struct Error {
    kind: ErrorKind,
    msg: Option<String>,
    source: Option<anyhow::Error>,
}

impl Error {
    pub fn with_message(mut self, msg: String) -> Self {
        self.msg = Some(msg);
        self
    }

    pub fn with_source<E: std::error::Error + std::fmt::Debug + Send + Sync + 'static>(
        mut self,
        source: E,
    ) -> Self {
        self.source = Some(source.into());
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let Some(msg) = self.msg.as_ref() {
            write!(f, "{}: {}", self.kind, msg)?;
        } else if let Some(source) = self.source.as_ref() {
            write!(f, "{}: {}", self.kind, source)?;
        } else {
            write!(f, "{}", self.kind)?;
        }
        Ok(())
    }
}
