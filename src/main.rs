use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file {}", &args.path.to_str().unwrap()))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }
    Ok(())
}
