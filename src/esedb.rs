use std::{fs::File, io::BufReader, path::Path};

use binrw::BinRead;
use getset::Getters;

use crate::{FileHeader, FileHeaderWithChecksum};

#[derive(Getters)]
#[getset(get = "pub")]
/// <https://github.com/libyal/libesedb/blob/main/documentation/Extensible%20Storage%20Engine%20(ESE)%20Database%20File%20(EDB)%20format.asciidoc>
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
