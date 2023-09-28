use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[allow(non_snake_case)]
mod myArgs;
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
        myArgs::EntryType::Create(CreateCommand { text }) => {}
    }
    Ok(())
}

fn findInFile(pattern: &String, path: &PathBuf) -> Result<()> {
    let file = File::open(&path)
        .with_context(|| format!("Could not read file {}", &path.to_str().unwrap()))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        if l.contains(pattern) {
            println!("{}", l);
        }
    }
    Ok(())
}
