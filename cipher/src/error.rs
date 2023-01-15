#![deny(missing_docs)]

use std::io;

/// Contains possible errors from the Cipher crate 
#[derive(Debug)]
pub enum CipherError {
    /// General unspecified error
    /// 
    /// To be deleted when not needed
    General(String),

    /// Reading from One time pad file was not succesful
    /// 
    /// converted from std::io::Error
    /// 
    KeyCouldNotBeRead(io::Error),

    /// number of bytes read was invalid
    InvalidKeyLen,

    /// The size of message was not valid
    InvalidMessageLen,
}

/// enables parsing from other error types
impl From<io::Error> for CipherError {
    /// std::io::Error -> CipherError::KeyCouldNotBeRead(std::io::Error)
    fn from(value: io::Error) -> Self {
        CipherError::KeyCouldNotBeRead(value)
    }
}

/// A custom Result type for CipherError
/// equivalent to Result<T, CipherError>
pub type Result<T> = core::result::Result<T, CipherError>;