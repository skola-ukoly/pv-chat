#[derive(Debug)]
pub enum SWError {
    BufferTooSmall,
    SocketFailure(std::io::Error),
    HeaderCouldNotBeParsed,
}

/// enables parsing from other error types
impl From<std::io::Error> for SWError {
    /// std::io::Error -> CipherError::KeyCouldNotBeRead(std::io::Error)
    fn from(value: std::io::Error) -> Self {
        SWError::SocketFailure(value)
    }
}

/// A custom Result type for SWError, equivalent to Result<T, SWError>
pub type Result<T> = core::result::Result<T, SWError>;