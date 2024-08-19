use binrw::binread;
use getset::Getters;

#[binread]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct DbTime {
    //#[br(assert(hours <= 23, "value for hours 0x{hours:02x} ({hours}) is too large"))]
    hours: u8,

    //#[br(assert(minutes <= 59, "value for minutes 0x{minutes:02x} ({minutes}) is too large"))]
    minutes: u8,

    //#[br(assert(seconds <= 59, "value for seconds 0x{seconds:02x} ({seconds}) is too large"))]
    seconds: u8,

    #[br(temp, count = 5)]
    #[getset(skip)]
    _padding: Vec<u8>,
}
