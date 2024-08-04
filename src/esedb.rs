use std::{fs::File, io::BufReader, path::Path};

use binrw::BinRead;
use getset::Getters;

use crate::{FileHeader, FileHeaderWithChecksum};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct EseDb {
    #[getset(skip)]
    reader: BufReader<File>,
    header: FileHeader,
}

impl EseDb {
    ///
    /// Open an EseDB database file
    pub fn open(filename: &Path) -> crate::Result<Self> {
        let mut reader = BufReader::new(File::open(filename)?);
        let header = FileHeaderWithChecksum::read_le(&mut reader)?.into_header();
        Ok(Self { reader, header })
    }
}
