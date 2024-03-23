use std::fmt::Display;

pub mod airprint;
pub mod messages;
pub mod proximity;

#[derive(Debug)]
pub enum ErrorKind {
    DecodeError,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            kind,
            message: Some(message.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {}",
            self.kind,
            self.message.as_deref().unwrap_or("")
        )
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error {
            kind: value,
            message: None,
        }
    }
}
