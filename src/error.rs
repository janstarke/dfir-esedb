use thiserror::Error;

pub type Result<T> = std::result::Result<T, EsedbError>;

#[derive(Error, Debug)]
pub enum EsedbError {
    #[error("an IO error has occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("a parser error has occurred: {0}")]
    ParserError(#[from] binrw::Error),

    #[error("invalid checksum! expected 0x{expected:08x} but found 0x{actual:08x} instead")]
    ChecksumError{expected: u32, actual: u32},

    #[error(transparent)]
    Other(#[from] anyhow::Error)
}