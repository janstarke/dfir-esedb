use binrw::binread;
use getset::Getters;

#[binread]
#[brw(little)]
#[derive(Getters)]
#[getset(get = "pub")]
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Storage/Jet/struct.JET_LOGTIME.html>
pub struct JetLogTime {
    #[br(assert(seconds <= 60, "value for `seconds` is too large: 0x{seconds:02x} ({seconds})"))]
    seconds: i8,

    #[br(assert(minutes <= 60))]
    minutes: i8,

    #[br(assert(hours <= 24))]
    hours: i8,

    #[br(assert(days <= 31))]
    days: i8,

    #[br(assert(months <= 12))]
    months: i8,

    #[br(temp)]
    #[getset(skip)]
    _years: i8,

    #[br(calc(i32::from(_years)+1900i32))]
    years: i32,

    #[br(temp)]
    #[getset(skip)]
    _filler: [u8; 2],
}
