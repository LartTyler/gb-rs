use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub file: PathBuf,

    /// If provided, skip `offset` bytes before beginning disassembly.
    #[arg(short, long, value_parser = clap_num::maybe_hex::<u16>)]
    pub offset: Option<u16>,

    /// If provided, do not enter interactive mode and output `limit` operations before
    /// terminating.
    #[arg(short, long, value_parser = clap_num::maybe_hex::<u16>)]
    pub limit: Option<u16>,

    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
