use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct MyArgs {
    #[clap(subcommand)]
    pub entry: EntryType,
}

#[derive(Debug, Subcommand)]
pub enum EntryType {
    Find(FindCommand),
    Create(CreateCommand),
}

#[derive(Debug, Args)]
pub struct FindCommand {
    pub pattern: String,
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub text: String,
}
