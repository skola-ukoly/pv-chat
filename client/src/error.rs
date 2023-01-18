use std::convert::From;

#[derive(Debug)]
pub enum ServerError {
    Generic,
    IOError,
    ParseError,
    PoisonError,
}

impl From<std::io::Error> for ServerError {
    fn from(_: std::io::Error) -> Self {
        Self::IOError
    }
}

impl From<std::str::Utf8Error> for ServerError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::ParseError
    }
}

impl<T> From<std::sync::PoisonError<T>> for ServerError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::PoisonError
    }
}
pub type Result<T> = std::result::Result<T, ServerError>;
