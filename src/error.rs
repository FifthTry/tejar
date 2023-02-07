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
}
