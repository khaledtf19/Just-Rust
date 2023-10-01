use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn findInFile(pattern: &String, path: &PathBuf) -> Result<()> {
    let mut liens: Vec<String> = vec![];
    let file = File::open(&path)
        .with_context(|| format!("Could not read file {}", &path.to_str().unwrap()))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line?;
        if l.contains(pattern) {
            liens.push(l)
        }
    }
    if liens.len() > 0 {
        liens.iter().for_each(|x| println!("{},", x))
    } else {
        println!("patteren does not match any line in the file")
    }
    Ok(())
}
