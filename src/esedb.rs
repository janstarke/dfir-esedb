use std::{fs::File, io::BufReader, path::Path};

pub struct EseDb {
    reader: BufReader<File>,
}

impl EseDb {
    pub fn open(filename: &Path) -> crate::Result<Self> {
        let reader = BufReader::new(File::open(filename)?);
        Ok(Self { reader })
    }
}
