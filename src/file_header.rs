use binrw::binread;
use getset::Getters;

use crate::{DatabaseState, DbTime, FileFormatRevision, FileFormatVersion, FileType, JetBkInfo, JetLgPos, JetLogTime, JetSignature};

#[binread]
#[brw(little, magic = b"\xef\xcd\xab\x89")]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct FileHeader {
    file_format_version: FileFormatVersion,
    file_type: FileType,
    database_time: DbTime,
    database_signature: JetSignature,
    database_state: DatabaseState,
    consistent_position: JetLgPos,
    was_consistent: JetLogTime,
    attached: JetLogTime,
    attach_position: JetLgPos,
    detached: JetLogTime,
    detach_position: JetLgPos,

    /// The JET_DBID data type contains the handle to the database. A database
    /// handle is used to manage the schema of a database. It can also be used
    /// to manage the tables inside of that database.
    dbid: u32,


    log_signature: JetSignature,
    previous_full_backup: JetBkInfo,
    previous_incremental_backup: JetBkInfo,
    current_fulll_backup: JetBkInfo,

    shadowing_disabled: u32,
    last_object_identifier: u32,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    service_pack_number: u32,
    file_format_revision: FileFormatRevision,

    page_size: u32,
    repair_count: u32,
    repaired: JetLogTime,

    #[br(temp, count=28)]
    #[getset(skip)]
    _unknown2: Vec<u8>,

    scrub_database_time: DbTime,
    scrubbed: JetLogTime,

    required_log: [u32; 2],

    #[br(temp)]
    #[getset(skip)]
    _upgrade_exchange_55_format: u32,

    #[br(temp)]
    #[getset(skip)]
    _upgrade_free_pages: u32,

    #[br(temp)]
    #[getset(skip)]
    _upgrade_space_map_pages: u32,

    current_shadow_backup: JetBkInfo,

    creation_file_format_version: FileFormatVersion,
    creation_file_format_revision: FileFormatRevision,

    #[br(temp, count=16)]
    #[getset(skip)]
    _unknown3: Vec<u8>,

    #[br(temp)]
    #[getset(skip)]
    _old_repair_count: u32,

    #[br(temp)]
    #[getset(skip)]
    _ecc_fix_success_count: u32,

    last_ecc_fix_success: JetLogTime,

    #[br(temp)]
    #[getset(skip)]
    _old_ecc_fix_success_count: u32,

    #[br(temp)]
    #[getset(skip)]
    _ecc_fix_error_count: u32,

    last_ecc_fix_error: JetLogTime,

    #[br(temp)]
    #[getset(skip)]
    _old_ecc_fix_error_count: u32,

    #[br(temp)]
    #[getset(skip)]
    _bad_checksum_error_count: u32,

    last_bad_checksum_error: JetLogTime,

    #[br(temp)]
    #[getset(skip)]
    _old_bad_checksum_error_count: u32,

    commited_log: u32,
    previous_shadow_copy_backup: JetBkInfo,
    previous_differential_backup: JetBkInfo,

    #[br(temp, count=40)]
    #[getset(skip)]
    _unknown4: Vec<u8>,

    nls_major_version: u32,
    nls_minor_version: u32,

    #[br(temp, count=148)]
    #[getset(skip)]
    _unknown5: Vec<u8>,

    _unknown_flags: u32

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
