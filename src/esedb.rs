use std::{
    fs::File,
    io::{BufReader, Seek},
    path::Path,
};

use anyhow::anyhow;
use binrw::{BinRead, BinReaderExt, VecArgs};
use getset::Getters;

use crate::{EsedbError, FileHeader, FileHeaderWithChecksum};

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
        let header = FileHeaderWithChecksum::read_le(&mut reader)?;
        let expected_checksum = *header.checksum();
        let header = header.into_header();
        let page_size = *header.page_size();

        if page_size % 4 != 0 {
            return Err(anyhow!("unexpected page size: {page_size}").into());
        }

        // step to the first byte after the magic value
        let ignore_bytes = 8;
        reader.seek(std::io::SeekFrom::Start(ignore_bytes.into()))?;

        let bytes: Vec<u32> = reader.read_le_args(VecArgs {
            count: ((page_size - ignore_bytes) / 4).try_into().unwrap(),
            inner: (),
        })?;

        let checksum = bytes.into_iter().fold(0, |x1, x2| x1 ^ x2);
        if checksum != expected_checksum {
            return Err(EsedbError::ChecksumError {
                actual: checksum,
                expected: expected_checksum,
            });
        }

        Ok(Self { reader, header })
    }
}
