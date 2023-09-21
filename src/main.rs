use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }
    Ok(())
}
