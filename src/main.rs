use flate2::read::ZlibDecoder;
use std::{
    fs,
    io::Read,
    path::{PathBuf, MAIN_SEPARATOR_STR},
};

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

const GIT_ROOT_FOLDER: &str = ".git";
const OBJECTS_FOLDER: &str = "objects";
const REFS_FOLDER: &str = "refs";
const HEAD_FILE: &str = "HEAD";

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
}

#[derive(Debug, Clone)]
struct Object {
    ///header
    header: String,
    ///checksum
    checksum: String,
}

impl Object {
    fn new(mut complete_checksum: String) -> Object {
        let checksum = complete_checksum.split_off(2);
        return Object {
            header: complete_checksum,
            checksum,
        };
    }

    fn print(&self) -> Result<()> {
        let mut path = PathBuf::from(GIT_ROOT_FOLDER);
        path.push(OBJECTS_FOLDER);
        path.push(&self.header);
        path.push(&self.checksum);
        let content = fs::read(path)?;
        let mut decoder = ZlibDecoder::new(content.as_slice());
        let mut content = String::new();
        decoder.read_to_string(&mut content)?;
        let (_, content) = content
            .split_once('\x00')
            .expect("Wrong file format for object");
        print!("{}", content);
        return Ok(());
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
            let object = Object::new(object);
            object.print()?;
        }
    }
    return Ok(());
}
