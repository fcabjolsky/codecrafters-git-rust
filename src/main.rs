use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use hex;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::{
    fs,
    io::Read,
    path::{PathBuf, MAIN_SEPARATOR_STR},
};

use anyhow::{Context, Ok, Result};
use clap::{Parser, Subcommand};

const GIT_ROOT_FOLDER: &str = ".git";
const OBJECTS_FOLDER: &str = "objects";
const REFS_FOLDER: &str = "refs";
const HEAD_FILE: &str = "HEAD";
const BLOB_HEADER: &str = "blob";

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    ///init
    Init,
    CatFile {
        #[arg(short = 'p')]
        object: String,
    },
    HashObject {
        #[arg(short = 'w')]
        file: String,
    },
}

#[derive(Debug, Clone)]
struct Object {
    ///header
    header: String,
    ///checksum
    checksum: String,
    ///content
    content: String,
}

impl Object {
    fn read(mut complete_checksum: String) -> Result<Object> {
        let checksum = complete_checksum.split_off(2);
        let mut path = PathBuf::from(GIT_ROOT_FOLDER);
        path.push(OBJECTS_FOLDER);
        path.push(&complete_checksum);
        path.push(&checksum);
        let content = fs::read(&path)
            .with_context(|| format!("Failed to read object {}{}", complete_checksum, checksum))?;
        let mut decoder = ZlibDecoder::new(content.as_slice());
        let mut content = String::new();
        decoder.read_to_string(&mut content)?;
        return Ok(Object {
            header: complete_checksum,
            checksum,
            content: String::from(content),
        });
    }

    fn print(&self) {
        let (_, content) = self
            .content
            .split_once('\x00')
            .expect("Wrong file format for object");
        print!("{}", content);
    }

    fn save(&self) -> Result<String> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(self.content.as_bytes())?;
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
impl From<PathBuf> for Object {
    fn from(path: PathBuf) -> Self {
        let file = fs::read_to_string(path).expect("File not found");
        let mut hasher = Sha1::new();
        let content = format!("{} {}\0{}", BLOB_HEADER, file.len(), file);
        hasher.update(&content);
        let hash = hasher.finalize();
        let mut header = hex::encode(hash).to_string();
        let checksum = header.split_off(2);
        return Object {
            header,
            checksum,
            content,
        };
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            fs::create_dir(GIT_ROOT_FOLDER)?;
            fs::create_dir(GIT_ROOT_FOLDER.to_owned() + MAIN_SEPARATOR_STR + OBJECTS_FOLDER)?;
            fs::create_dir(GIT_ROOT_FOLDER.to_owned() + MAIN_SEPARATOR_STR + REFS_FOLDER)?;
            fs::write(
                GIT_ROOT_FOLDER.to_owned() + MAIN_SEPARATOR_STR + HEAD_FILE,
                "ref: refs/heads/master\n",
            )?;
        }
        Command::CatFile { object } => {
            let object = Object::read(object)?;
            object.print();
        }
        Command::HashObject { file } => {
            let path = PathBuf::from(file);
            let object = Object::from(path);
            let hash_object = object.save()?;
            print!("{}", hash_object);
        }
    }
    return Ok(());
}
