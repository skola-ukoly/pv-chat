#[derive(Debug)]
pub enum ChatError {
    Generic,
    IOError(std::io::Error),
}

impl From<std::io::Error> for ChatError {
    fn from(e: std::io::Error) -> ChatError {
        ChatError::IOError(e)
    }
}
