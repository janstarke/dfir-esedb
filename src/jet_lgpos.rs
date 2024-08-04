use binrw::binread;
use getset::Getters;

#[binread]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct JetLgPos {
    block: u16,
    sector: u16,
    generation: u32
}
