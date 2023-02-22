pub use read::*;
pub use write::*;

use std::{fmt::Display, str::FromStr};

mod read;
mod write;

#[derive(Debug)]
pub enum Command {
    Next,
    Reset,
    CartInfo,
    Quit,
    ReadByte(ReadByteCommand),
    ReadWord(ReadWordCommand),
    WriteByte(WriteByteCommand),
    WriteWord(WriteWordCommand),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut args: Vec<_> = s.split_whitespace().rev().collect();
        let Some(command) = args.pop() else {
            return Err(Error::Empty);
        };

        match command.to_lowercase().as_str() {
            "q" | "quit" => Ok(Self::Quit),
            "n" | "next" => Ok(Self::Next),
            "r" | "reset" => Ok(Self::Reset),
            "ci" | "cart-info" => Ok(Self::CartInfo),
            "rb" | "read-byte" => ReadByteCommand::from_args(args),
            "rw" | "read-word" => ReadWordCommand::from_args(args),
            "wb" | "write-byte" => WriteByteCommand::from_args(args),
            "ww" | "write-word" => WriteWordCommand::from_args(args),
            _ => Err(Error::Unrecognized),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quit => f.write_str("quit"),
            Self::Next => f.write_str("next"),
            Self::Reset => f.write_str("reset"),
            Self::CartInfo => f.write_str("cart-info"),
            Self::ReadByte(inner) => {
                f.write_str("read-byte ")?;
                f.write_fmt(format_args!("{inner}"))
            }
            Self::ReadWord(inner) => {
                f.write_str("read-word ")?;
                f.write_fmt(format_args!("{inner}"))
            }
            Self::WriteByte(inner) => {
                f.write_str("write-byte ")?;
                f.write_fmt(format_args!("{inner}"))
            }
            Self::WriteWord(inner) => {
                f.write_str("write-word ")?;
                f.write_fmt(format_args!("{inner}"))
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty command")]
    Empty,

    #[error("unrecognized command")]
    Unrecognized,

    #[error("received one or more invalid arguments")]
    InvalidArgument,

    #[error("missing one or more required arguments")]
    MissingArgument,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait FromArgs {
    /// Parses a vec of string arguments, split on whitespace characters, into a [`Command`].
    fn from_args(args: Vec<&str>) -> Result<Command>;
}

/// Utility function for forwarding [`str::parse()`] calls and converting any errors into
/// appropriate command [`Error`] variants.
fn parse_arg<T: FromStr>(value: Option<&str>) -> Result<T> {
    match value {
        Some(value) => value.parse().map_err(|_| Error::InvalidArgument),
        _ => Err(Error::MissingArgument),
    }
}
