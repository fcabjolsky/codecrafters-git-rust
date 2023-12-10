use anyhow::{Ok, Result};

use hex;
use sha1::{Digest, Sha1};
use std::{fs, path::PathBuf};

use crate::object::Object;

const BLOB_HEADER: &str = "blob";

#[derive(Debug, Clone)]
pub struct Blob {
    object: Object,
}

impl From<PathBuf> for Blob {
    fn from(path: PathBuf) -> Self {
        let file = fs::read_to_string(path).expect("File not found");
        let mut hasher = Sha1::new();
        let content = format!("{} {}\0{}", BLOB_HEADER, file.len(), file);
        hasher.update(&content);
        let hash = hasher.finalize();
        let mut header = hex::encode(hash).to_string();
        let checksum = header.split_off(2);
        return Blob {
            object: Object {
                header,
                checksum,
                content: content.as_bytes().to_vec(),
            },
        };
    }
}

impl Blob {
    pub fn print(&self) {
        let content = String::from_utf8(self.object.content.clone()).unwrap_or_default();
        let (_, content) = content
            .split_once('\x00')
            .expect("Wrong file format for object");
        print!("{}", content);
    }

    pub fn read(complete_checksum: String) -> Result<Self> {
        return Ok(Blob {
            object: Object::read(complete_checksum)?,
        });
    }

    pub fn save(&self) -> Result<String> {
        self.object.save()
    }
}
