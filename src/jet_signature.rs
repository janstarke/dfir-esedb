use std::io::Cursor;

use binrw::{binread, BinReaderExt, NullString};
use getset::Getters;

use crate::JetLogTime;

#[binread]
#[derive(Getters)]
#[getset(get = "pub")]
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Storage/Jet/struct.JET_SIGNATURE.html>
pub struct JetSignature {
    /// A randomly assigned number
    random: u32,

    creation_date_and_time: JetLogTime,

    #[br(temp, count=16)]
    #[getset(skip)]
    computer_name_data: Vec<u8>,

    /// The NetBIOS computer name
    #[br(calc(Cursor::new(computer_name_data).read_le::<NullString>()?.to_string()))]
    computer_name: String,
}
