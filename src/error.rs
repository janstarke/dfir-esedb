use thiserror::Error;

pub type Result<T> = std::result::Result<T, EsedbError>;

#[derive(Error, Debug)]
pub enum EsedbError {
    #[error("an IO error has occurred: {0}")]
    IoError(std::io::Error),

    #[error("a parser error has occurred: {0}")]
    ParserError(binrw::Error),
}

impl From<std::io::Error> for EsedbError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<binrw::Error> for EsedbError {
    fn from(value: binrw::Error) -> Self {
        Self::ParserError(value)
    }
}