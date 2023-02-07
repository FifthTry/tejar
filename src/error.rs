#[derive(thiserror::Error, Debug)]
pub enum CreateError {
    #[error("data store disconnected")]
    IoError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ReadError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("list content parse error: {line}, {message}")]
    ParseError { line: usize, message: String },
    #[error("NotFound: {0}")]
    NotFound(String),
    #[error("OutOfRange: {0}")]
    OutOfRange(String),
    #[error("from utf8 error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
