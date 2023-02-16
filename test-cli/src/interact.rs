use dialoguer::{theme::ColorfulTheme, Input};
use std::str::FromStr;

pub struct Interact {
    theme: ColorfulTheme,
    previous_command: Option<Command>,
}

impl Interact {
    pub fn new() -> Self {
        let theme = ColorfulTheme {
            prompt_prefix: console::style("".into()),
            prompt_suffix: console::style("> ".into()).black().bright(),
            ..Default::default()
        };

        Self {
            theme,
            previous_command: None,
        }
    }

    pub fn prompt(&mut self) -> Result<Command, InteractError> {
        let default_command = self.previous_command.take().unwrap_or(Command::Next);

        let input: String = Input::with_theme(&self.theme)
            .default(default_command.as_str().to_string())
            .interact_text()?;

        let args: Vec<_> = input.split_whitespace().rev().collect();
        let command: Command = args.try_into()?;

        if command.is_history_allowed() {
            self.previous_command = Some(command.clone());
        }

        Ok(command)
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Next,
    Memory(u16, MemoryReadKind),
    Convert {
        original: String,
        value: u16,
        format: OutputFormat,
    },
    Info,
    Quit,
}

impl Command {
    pub fn is_history_allowed(&self) -> bool {
        matches!(self, Self::Next)
    }
}

impl TryFrom<Vec<&str>> for Command {
    type Error = InteractError;

    fn try_from(mut source: Vec<&str>) -> Result<Self, Self::Error> {
        let Some(input) = source.pop() else {
            return Err(InteractError::Empty);
        };

        let command = match input {
            "q" | "quit" => Self::Quit,
            "n" | "next" => Self::Next,
            "i" | "info" => Self::Info,
            "c" | "convert" => {
                let Some(original) = source.pop() else {
                    return Err(InteractError::UnrecognizedCommand);
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
            "m" | "mem" | "memory" => {
                let Some(addr) = source.pop() else {
                    return Err(InteractError::UnrecognizedCommand);
                };

                let addr = convert_numeric_input(addr)?;
                let kind = if let Some(kind) = source.pop() {
                    match kind {
                        "b" | "byte" => MemoryReadKind::Byte,
                        "w" | "word" => MemoryReadKind::Word,
                        _ => return Err(InteractError::UnrecognizedCommand),
                    }
                } else {
                    MemoryReadKind::Byte
                };

                Self::Memory(addr, kind)
            }
            _ => return Err(InteractError::UnrecognizedCommand),
        };

        Ok(command)
    }
}

impl Command {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Quit => "quit",
            Self::Next => "next",
            Self::Convert { .. } => "convert",
            Self::Memory(_, _) => "memory",
            Self::Info => "info",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InteractError {
    #[error("interact error: {0}")]
    IO(#[from] std::io::Error),

    #[error("interact error: empty command")]
    Empty,

    #[error("interact error: unrecognized command")]
    UnrecognizedCommand,

    #[error("interact error: invalid input")]
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryReadKind {
    Byte,
    Word,
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
    type Err = InteractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "h" | "hex" => Self::Hex,
            "d" | "dec" | "decimal" => Self::Decimal,
            "b" | "bin" | "binary" => Self::Binary,
            _ => return Err(InteractError::InvalidInput),
        };

        Ok(result)
    }
}

fn convert_numeric_input(value: &str) -> Result<u16, InteractError> {
    let (fragment, radix) = match &value[0..2] {
        "0x" => (&value[2..], 16),
        "0b" => (&value[2..], 2),
        "0o" => (&value[2..], 7),
        _ => (value, 10),
    };

    u16::from_str_radix(fragment, radix).map_err(|_| InteractError::InvalidInput)
}
