mod blob;
mod object;
mod tree;

use std::{
    fs,
    path::{PathBuf, MAIN_SEPARATOR_STR},
};

use anyhow::{Ok, Result};
use blob::Blob;
use clap::{Parser, Subcommand};
use object::{GIT_ROOT_FOLDER, HEAD_FILE, OBJECTS_FOLDER, REFS_FOLDER};
use tree::Tree;

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
    LsTree {
        #[arg(short, long)]
        name_only: String,
    },
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
            let blob = Blob::read(object)?;
            blob.print();
        }
        Command::HashObject { file } => {
            let path = PathBuf::from(file);
            let blob = Blob::from(path);
            let hash_object = blob.save()?;
            print!("{}", hash_object);
        }
        Command::LsTree { name_only } => {
            let tree = Tree::read(name_only)?;
            tree.print();
        }
    }
    return Ok(());
}
