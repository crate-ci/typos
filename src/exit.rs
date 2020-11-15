pub struct ExitCode {
    pub code: sysexit::Code,
    pub error: Option<anyhow::Error>,
}

impl ExitCode {
    pub fn code(code: sysexit::Code) -> Self {
        Self { code, error: None }
    }

    pub fn error(mut self, error: anyhow::Error) -> Self {
        self.error = Some(error);
        self
    }
}

impl From<sysexit::Code> for ExitCode {
    fn from(code: sysexit::Code) -> Self {
        Self::code(code)
    }
}

pub trait ChainCodeExt {
    fn error(self) -> ExitCode;
    fn chain(self, error: anyhow::Error) -> ExitCode;
}

impl ChainCodeExt for sysexit::Code {
    fn error(self) -> ExitCode {
        ExitCode::code(self)
    }
    fn chain(self, error: anyhow::Error) -> ExitCode {
        ExitCode::code(self).error(error)
    }
}

pub trait ExitCodeResultErrorExt<T> {
    fn code(self, code: sysexit::Code) -> Result<T, ExitCode>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ExitCodeResultErrorExt<T> for Result<T, E> {
    fn code(self, code: sysexit::Code) -> Result<T, ExitCode> {
        self.map_err(|e| ExitCode::code(code).error(e.into()))
    }
}

pub trait ExitCodeResultAnyhowExt<T> {
    fn code(self, code: sysexit::Code) -> Result<T, ExitCode>;
}

impl<T> ExitCodeResultAnyhowExt<T> for Result<T, anyhow::Error> {
    fn code(self, code: sysexit::Code) -> Result<T, ExitCode> {
        self.map_err(|e| ExitCode::code(code).error(e))
    }
}
