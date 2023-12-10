use std::{path::PathBuf, fs};

use anyhow::{Ok, Result, Context};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::prelude::*;


pub const GIT_ROOT_FOLDER: &str = ".git";
pub const OBJECTS_FOLDER: &str = "objects";
pub const REFS_FOLDER: &str = "refs";
pub const HEAD_FILE: &str = "HEAD";

#[derive(Debug, Clone)]
pub struct Object {
    ///header
    pub header: String,
    ///checksum
    pub checksum: String,
    ///content
    pub content: Vec<u8>,
}

impl Object {
    pub fn read(mut complete_checksum: String) -> Result<Object> {
        let checksum = complete_checksum.split_off(2);
        let mut path = PathBuf::from(GIT_ROOT_FOLDER);
        path.push(OBJECTS_FOLDER);
        path.push(&complete_checksum);
        path.push(&checksum);
        let content = fs::read(&path)
            .with_context(|| format!("Failed to read object {}{}", complete_checksum, checksum))?;
        let mut decoder = ZlibDecoder::new(content.as_slice());
        let mut content = vec![];
        decoder.read_to_end(&mut content)?;
        return Ok(Object {
            header: complete_checksum,
            checksum,
            content,
        });
    }

    pub fn save(&self) -> Result<String> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(self.content.as_slice())?;
        let compressed = e.finish()?;
        let mut path = PathBuf::from(GIT_ROOT_FOLDER);
        path.push(OBJECTS_FOLDER);
        path.push(&self.header);
        fs::create_dir_all(&path)?;
        path.push(&self.checksum);
        fs::write(path, compressed)?;
        return Ok(format!("{}{}", self.header, self.checksum));
    }
}
