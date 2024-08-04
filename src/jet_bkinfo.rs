use binrw::binread;
use getset::Getters;

use crate::{JetLgPos, JetLogTime};

#[binread]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct JetBkInfo {
    /// Contains an identifier of the backup
    backup_position: JetLgPos,

    /// The backup creation date and time
    created: JetLogTime,

    /// The lower log generation number associated with the backup.
    generation_lower_number: u32,

    ///  The upper log generation number associated with the backup.
    generation_upper_number: u32
}
