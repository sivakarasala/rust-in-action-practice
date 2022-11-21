use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, WriteBytesExt};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use serde_derive::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }
}

pub fn load(&mut self) -> io::Result<()> {
    let mut f = BufReader::new(&mut self.f);

    loop {
        let position = f.seek(SeekFrom::Current(0))?;

        let maybe_kv = ActionKV::process_record(&mut f);

        let kv = match maybe_kv {
            Ok(kv) => kv,
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    },
                    _ => return Err(err)
                }
            }
        };

        self.index.insert(kv.key, position);
    }

    Ok(())
}
