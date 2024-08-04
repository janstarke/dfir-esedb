use binrw::binread;
use getset::Getters;

use crate::{DbTime, FileFormatVersion, FileType, JetSignature};

#[binread]
#[brw(little, magic = b"\xef\xcd\xab\x89")]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct FileHeader {
    file_format_version: FileFormatVersion,
    file_type: FileType,
    database_time: DbTime,
    database_signature: JetSignature,
}

#[binread]
#[brw(little)]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct FileHeaderWithChecksum {
    /// The checksum is a XOR over the 32-bit little-endian values in the header
    /// starting at offset 8 to at least offset 668, but presumably page size.
    /// The value 0x89abcdef is used as the initial value.
    checksum: u32,
    header: FileHeader,
}

impl FileHeaderWithChecksum {
    pub fn into_header(self) -> FileHeader {
        self.header
    }
}
