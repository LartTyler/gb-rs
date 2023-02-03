use clap::{Parser, ValueEnum};
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

    #[arg(long, value_enum, default_value_t = LimitUnit::Instructions)]
    pub limit_on: LimitUnit,

    #[arg(short, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LimitUnit {
    Instructions,
    Bytes,
}
