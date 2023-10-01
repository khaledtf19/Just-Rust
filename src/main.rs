use std::path::PathBuf;
use std::{fs::File, io::Write};

use anyhow::{Context, Result};
use clap::Parser;

mod find;
#[allow(non_snake_case)]
mod myArgs;
use find::findInFile;
use myArgs::{CreateCommand, FindCommand, MyArgs};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = MyArgs::parse();
    let entry = &args.entry;
    match entry {
        myArgs::EntryType::Find(FindCommand { pattern, path }) => {
            findInFile(pattern, path)?;
        }
        myArgs::EntryType::Create(CreateCommand { text, path }) => {
            createFile(text, path)?;
        }
    }
    Ok(())
}

fn createFile(text: &String, path: &PathBuf) -> Result<()> {
    let mut f = File::create(&path)
        .with_context(|| format!("Could not create file: {} ", &path.to_str().unwrap()))?;
    f.write(text.as_bytes())?;
    Ok(())
}
