use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub file: PathBuf,
}
