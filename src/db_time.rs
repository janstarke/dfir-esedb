use binrw::binread;
use getset::Getters;

#[binread]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct DbTime {
    #[br(assert(hours <= 23, "value 0x{:04x} is too large", hours))]
    hours: u8,

    #[br(assert(minutes <= 59, "value 0x{:04x} is too large", minutes))]
    minutes: u8,

    #[br(assert(seconds <= 59, "value 0x{:04x} is too large", seconds))]
    seconds: u8,

    #[br(temp, count = 5)]
    #[getset(skip)]
    _padding: Vec<u8>,
}
