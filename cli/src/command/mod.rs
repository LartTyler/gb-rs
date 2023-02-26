pub use auto::*;
pub use read::*;
pub use write::*;

use std::{fmt::Display, num::ParseIntError, str::FromStr};

mod auto;
mod read;
mod write;

#[derive(Debug, Clone)]
pub enum Command {
    Next,
    Reset,
    CartInfo,
    Quit,
    Auto(AutoCommand),
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
            "a" | "auto" => AutoCommand::from_args(args),
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
            Self::Auto(inner) => {
                f.write_str("auto ")?;
                f.write_fmt(format_args!("{inner}"))
            }
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
fn parse_arg<T: ParseNumber>(value: Option<&str>) -> Result<T> {
    match value {
        Some(value) => T::parse_number(value).map_err(|_| Error::InvalidArgument),
        _ => Err(Error::MissingArgument),
    }
}

trait ParseNumber: Sized {
    type Err = ParseIntError;

    fn parse_number<S: AsRef<str>>(input: S) -> std::result::Result<Self, Self::Err>;

    fn guess_radix(input: &str) -> (&str, u32) {
        if input.len() >= 3 {
            let radix = match &input[0..2] {
                "0x" => 16,
                "0b" => 2,
                "0o" => 7,
                _ => return (input, 10),
            };

            (&input[2..], radix)
        } else {
            (input, 10)
        }
    }
}

impl ParseNumber for u8 {
    fn parse_number<S: AsRef<str>>(input: S) -> std::result::Result<Self, Self::Err> {
        let (fragment, radix) = Self::guess_radix(input.as_ref());
        Self::from_str_radix(fragment, radix)
    }
}

impl ParseNumber for u16 {
    fn parse_number<S: AsRef<str>>(input: S) -> std::result::Result<Self, Self::Err> {
        let (fragment, radix) = Self::guess_radix(input.as_ref());
        Self::from_str_radix(fragment, radix)
    }
}

impl ParseNumber for u64 {
    fn parse_number<S: AsRef<str>>(input: S) -> std::result::Result<Self, Self::Err> {
        let (fragment, radix) = Self::guess_radix(input.as_ref());
        Self::from_str_radix(fragment, radix)
    }
}
