use binrw::binrw;

#[binrw]
#[brw(little, repr=u32)]
pub enum FileFormatVersion {
    /// Original operating system Beta format (April 22, 1997).
    Original = 0x00000620,

    /// New Space Manager (May 15, 1999).
    NewSpaceManager = 0x00000623,
}
