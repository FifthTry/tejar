
#[derive(thiserror::Error, Debug)]
pub enum CreateError{
    #[error("data store disconnected")]
    IoError(#[from] std::io::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ReadError{
    #[error("data store disconnected")]
    IoError(#[from] std::io::Error),
}