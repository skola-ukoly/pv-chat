
#[derive(Debug)]
pub enum ServerError {
    Generic,
    IOError(std::io::Error),
    ParseError,
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        ServerError::IOError(e)
    }
}

impl From<config::ConfigError> for ServerError {
    fn from(_: config::ConfigError) -> ServerError {
        ServerError::ParseError
    }
}

pub type Result<T> = std::result::Result<T, ServerError>;