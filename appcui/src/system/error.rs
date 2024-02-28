use std::fmt::Display;
#[cfg(target_family = "unix")]
use crate::terminals::ansi::TermiosError;

#[repr(u8)]
#[derive(Debug)]
pub enum ErrorKind {
    InitializationFailure,
    InvalidFeature,
    InvalidParameter,
    #[cfg(target_family = "unix")]
    TermiosError(TermiosError),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    // Note: Do we really need this string stored?
    description: String,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, description: String) -> Self {
        Self { kind, description }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.description)
    }
}

#[cfg(target_family = "unix")]
impl From<TermiosError> for Error {
    fn from(err: TermiosError) -> Self {
        Self {
            kind: ErrorKind::TermiosError(err),
            description: "Termios setup in terminal failed".to_string(),
        }
    }
}
