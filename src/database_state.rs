use binrw::binread;

#[binread]
#[br(repr=u32)]
#[allow(non_camel_case_types)]
pub enum DatabaseState {
    /// The database was just created.
    JET_dbstateJustCreated = 1,

    /// The database requires hard or soft recovery to be run in order to become
    /// usable or movable. One should not try to move databases in this state.
    JET_dbstateDirtyShutdown = 2,

    /// The database is in a clean state. The database can be attached without
    /// any log files.
    JET_dbstateCleanShutdown = 3,

    /// The database is being upgraded.
    JET_dbstateBeingConverted = 4,

    /// Internal.
    /// This value is introduced in Windows XP
    JET_dbstateForceDetach
}