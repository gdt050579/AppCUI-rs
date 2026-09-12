use std::fmt::Display;

/// Category of a system [`Error`].
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// A subsystem failed to start (terminal backend, runtime, or similar).
    InitializationFailure,
    /// The requested feature is not available on this platform or build.
    InvalidFeature,
    /// A method received an invalid argument.
    InvalidParameter,
}

/// An error produced while creating or running an [`super::App`].
///
/// `kind` classifies the failure. [`std::fmt::Display`] prints the human-readable description.
#[derive(Debug)]
pub struct Error {
    /// The category of the failure.
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
        write!(f, "{}", self.description)
    }
}
