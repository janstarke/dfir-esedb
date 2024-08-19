use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Cursor, ErrorKind, Read, Seek},
    path::Path,
    rc::Rc,
};

use anyhow::anyhow;
use binrw::{BinRead, BinReaderExt, VecArgs};
use getset::Getters;

use crate::{EsedbError, FileHeader, FileHeaderWithChecksum, Page};

#[derive(Getters)]
#[getset(get = "pub")]
/// <https://github.com/libyal/libesedb/blob/main/documentation/Extensible%20Storage%20Engine%20(ESE)%20Database%20File%20(EDB)%20format.asciidoc>
pub struct EseDb {
    #[getset(skip)]
    _reader: BufReader<File>,
    header: FileHeader,
    pages: HashMap<u64, Rc<Page>>,
}

impl EseDb {
    ///
    /// Open an EseDB database file
    pub fn open(filename: &Path) -> crate::Result<Self> {
        let mut reader = BufReader::new(File::open(filename)?);
        let header = FileHeaderWithChecksum::read_le(&mut reader)?;
        let expected_checksum = *header.checksum();
        let header = header.into_header();
        let page_size = *header.page_size();

        if page_size % 4 != 0 {
            return Err(anyhow!("unexpected page size: {page_size}").into());
        }

        // step to the first byte after the magic value
        let ignore_bytes = 8;
        reader.seek(std::io::SeekFrom::Start(ignore_bytes.into()))?;

        let bytes: Vec<u32> = reader.read_le_args(VecArgs {
            count: ((page_size - ignore_bytes) / 4).try_into().unwrap(),
            inner: (),
        })?;

        let checksum = bytes.into_iter().fold(0, |x1, x2| x1 ^ x2);
        if checksum != expected_checksum {
            return Err(EsedbError::ChecksumError {
                actual: checksum,
                expected: expected_checksum,
            });
        }

        // read all Pages
        let mut pages = HashMap::new();
        let mut buffer = vec![0u8; *header.page_size() as usize];
        //let mut next_page_offset = reader.stream_position()?;
        loop {
            let page_offset = reader.stream_position()?;

            if let Err(why) = reader.read_exact(&mut buffer) {
                if why.kind() == ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(why.into());
                }
            }

            let mut page_reader = Cursor::new(&buffer);
            let page = page_reader.read_le_args::<Page>((
                *header.page_size(),
                *header.file_format_version(),
                *header.file_format_revision(),
            ))?;

            pages.insert(page_offset, Rc::new(page));
        }

        Ok(Self {
            _reader: reader,
            header,
            pages,
        })
    }
}
