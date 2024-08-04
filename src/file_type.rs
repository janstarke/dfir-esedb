use binrw::binrw;

#[binrw]
#[brw(little, repr=u32)]
pub enum FileType {
    /// Contains a hierarchical page-based storage
    Database = 0,

    /// Contains streamed data.
    StreamingFile = 1,
}
