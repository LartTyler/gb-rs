use dialoguer::{theme::ColorfulTheme, Input};

pub struct Interact {
    theme: ColorfulTheme,
    previous_command: Option<Command>,
}

impl Interact {
    pub fn new() -> Self {
        let mut theme = ColorfulTheme::default();
        theme.prompt_prefix = console::style("".into());
        theme.prompt_suffix = console::style("> ".into()).black().bright();

        Self {
            theme,
            previous_command: None,
        }
    }

    pub fn prompt(&mut self) -> Result<Command, InteractError> {
        let default_command = self.previous_command.unwrap_or(Command::Next);

        let input: String = Input::with_theme(&self.theme)
            .default(default_command.as_str().to_string())
            .interact_text()?;

        let args: Vec<_> = input.split_whitespace().rev().collect();
        let command: Command = args.try_into()?;

        if command.is_history_allowed() {
            self.previous_command = Some(command);
        }

        Ok(command)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryReadKind {
    Byte,
    Word,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Next,
    Memory(u16, MemoryReadKind),
    Info,
    Quit,
}

impl Command {
    pub fn is_history_allowed(&self) -> bool {
        match self {
            Self::Next => true,
            _ => false,
        }
    }
}

impl TryFrom<Vec<&str>> for Command {
    type Error = InteractError;

    fn try_from(mut value: Vec<&str>) -> Result<Self, Self::Error> {
        let Some(input) = value.pop() else {
            return Err(InteractError::Empty);
        };

        let command = match input {
            "q" | "quit" => Self::Quit,
            "n" | "next" => Self::Next,
            "i" | "info" => Self::Info,
            "m" | "mem" | "memory" => {
                let Some(addr) = value.pop() else {
                    return Err(InteractError::UnrecognizedCommand);
                };

                let addr = match &addr[0..2] {
                    "0x" => u16::from_str_radix(&addr[2..], 16),
                    "0b" => u16::from_str_radix(&addr[2..], 2),
                    "0o" => u16::from_str_radix(&addr[2..], 7),
                    _ => addr.parse(),
                }
                .map_err(|_| InteractError::UnrecognizedCommand)?;

                let kind = if let Some(kind) = value.pop() {
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
}
