use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    InitializationFailure,
    InvalidFeature,
    InvalidParameter,
}
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
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
