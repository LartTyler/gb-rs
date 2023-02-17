use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty command")]
    Empty,

    #[error("unrecognized command")]
    Unrecognized,

    #[error("missing one or more arguments")]
    Incomplete,

    #[error("received one or more invalid arguments")]
    InvalidArgument,
}

#[derive(Debug, Clone)]
pub enum Command {
    Next,
    Reset,
    CartInfo,
    Read(u16, ReadKind),
    Write(u16, WriteKind),
    Convert {
        original: String,
        value: u16,
        format: OutputFormat,
    },
    Quit,
}

impl Command {
    pub fn is_history_allowed(&self) -> bool {
        matches!(self, Self::Next)
    }
}

impl TryFrom<Vec<&str>> for Command {
    type Error = Error;

    fn try_from(mut source: Vec<&str>) -> Result<Self, Self::Error> {
        let Some(input) = source.pop() else {
            return Err(Error::Empty);
        };

        let command = match input {
            "q" | "quit" => Self::Quit,
            "n" | "next" => Self::Next,
            "reset" => Self::Reset,
            "cart" => Self::CartInfo,
            "c" | "convert" => {
                let Some(original) = source.pop() else {
                    return Err(Error::Incomplete);
                };

                let value = convert_numeric_input(original)?;
                let format = match source.pop() {
                    Some(f) => f.parse()?,
                    None => OutputFormat::Decimal,
                };

                Self::Convert {
                    original: original.to_owned(),
                    value,
                    format,
                }
            }
            "r" | "ra" | "read" | "read-addr" => {
                let Some(kind) = source.pop() else {
                    return Err(Error::Incomplete);
                };

                let kind = match kind {
                    "b" | "byte" => ReadKind::Byte,
                    "w" | "word" => ReadKind::Word,
                    _ => return Err(Error::InvalidArgument),
                };

                let Some(addr) = source.pop() else {
                    return Err(Error::Incomplete);
                };

                let addr = convert_numeric_input(addr)?;

                Self::Read(addr, kind)
            }
            // write-addr "byte" | "word" addr value
            "w" | "wa" | "write" | "write-addr" => {
                // First argument is always the type of value to write. Grab that out, then hold
                // onto it for when we parse out the `value` argument.
                let Some(kind) = source.pop() else {
                    return Err(Error::Incomplete);
                };

                // Second argument is the address to write to. We can snag that and immediately
                // parse it.
                let Some(addr) = source.pop() else {
                    return Err(Error::Incomplete);
                };

                let addr = convert_numeric_input(addr)?;

                // Third and final argument is the value to write. After grabbing it, we can use
                // the `kind` argument from earlier to determine what type of value is being
                // written.
                let Some(value) = source.pop() else {
                    return Err(Error::Incomplete);
                };

                let value = convert_numeric_input(value)?;

                // The `kind` argument we're matching against here was parsed out at the beginning
                // of this block. I kinda hate that we read it then don't use it for so long, but
                // I need to do some major cleanup of command parsing anyway, so it'll just stay a
                // bit of a mess until I get around to that.
                let kind = match kind {
                    "b" | "byte" => {
                        let value: u8 = value.try_into().map_err(|_| Error::InvalidArgument)?;
                        WriteKind::Byte(value)
                    }
                    "w" | "word" => WriteKind::Word(value),
                    _ => return Err(Error::InvalidArgument),
                };

                Self::Write(addr, kind)
            }
            _ => return Err(Error::Unrecognized),
        };

        Ok(command)
    }
}

impl Command {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Quit => "quit",
            Self::Next => "next",
            Self::Reset => "reset",
            Self::CartInfo => "cart",
            Self::Convert { .. } => "convert",
            Self::Read(_, _) => "read-addr",
            Self::Write(_, _) => "write-addr",
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())?;

        match self {
            Self::Convert {
                original, format, ..
            } => f.write_fmt(format_args!(" {original} {format}")),
            Self::Read(addr, kind) => f.write_fmt(format_args!(" {kind} ${addr:04X}")),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ReadKind {
    Byte,
    Word,
}

impl Display for ReadKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Byte => "byte",
            Self::Word => "word",
        })
    }
}

#[derive(Debug, Clone)]
pub enum WriteKind {
    Byte(u8),
    Word(u16),
}

impl Display for WriteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Byte(n) => n.to_string(),
            Self::Word(n) => n.to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Decimal,
    Hex,
    Binary,
}

impl OutputFormat {
    pub fn as_formatted(&self, value: u16) -> String {
        match self {
            Self::Decimal => format!("{value:>5}"),
            Self::Hex => {
                if value > 0xFF {
                    format!("{value:#06X}")
                } else {
                    format!("{value:#04X}")
                }
            }
            Self::Binary => {
                if value > 0xFF {
                    format!("{value:#018b}")
                } else {
                    format!("{value:#010b}")
                }
            }
        }
    }
}

impl FromStr for OutputFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "h" | "hex" => Self::Hex,
            "d" | "dec" | "decimal" => Self::Decimal,
            "b" | "bin" | "binary" => Self::Binary,
            _ => return Err(Error::InvalidArgument),
        };

        Ok(result)
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Hex => "hex",
            Self::Binary => "bin",
            Self::Decimal => "dec",
        })
    }
}

fn convert_numeric_input(value: &str) -> Result<u16, Error> {
    let (fragment, radix) = if value.len() >= 3 {
        match &value[0..2] {
            "0x" => (&value[2..], 16),
            "0b" => (&value[2..], 2),
            "0o" => (&value[2..], 7),
            _ => (value, 10),
        }
    } else {
        (value, 10)
    };

    u16::from_str_radix(fragment, radix).map_err(|_| Error::InvalidArgument)
}
