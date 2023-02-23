use crate::command::{self, Command};
use crossterm::event::{self, Event, KeyCode};
use gb_rs_asm::operations::OperationKind;
use gb_rs_core::{cpu::inspector::Message, memory::cartridge::mbc::ControllerType, Hardware};
use std::{
    path::Path,
    sync::mpsc::{Receiver, TryRecvError},
};

pub type Result<T> = std::result::Result<T, Error>;

pub struct App {
    pub last_command: Option<Command>,
    pub hardware: Hardware,
    pub input: String,
    operation_log: Vec<CpuStepResult>,
    command_log: Vec<CommandResult>,
    inspector_rx: Receiver<Message>,
}

impl App {
    const MAX_OPERATION_LOG_LEN: usize = 50;
    const MAX_COMMAND_LOG_LEN: usize = 50;

    pub fn from_file(cart_file: &Path) -> Result<Self> {
        let mut hardware = Hardware::from_file(cart_file)?;

        Ok(Self {
            inspector_rx: hardware.cpu.inspect(),
            hardware,
            last_command: Some(Command::Next),
            operation_log: Vec::with_capacity(Self::MAX_OPERATION_LOG_LEN),
            command_log: Vec::with_capacity(Self::MAX_COMMAND_LOG_LEN),
            input: String::with_capacity(16),
        })
    }

    pub fn run(&mut self) -> Result<Outcome> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => self.input.push(c),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    let outcome = self.on_command()?;

                    // Return early if we're quitting or resetting; there's no reason to continue
                    // the main application loop
                    if matches!(outcome, Outcome::Quit | Outcome::Reset) {
                        return Ok(outcome);
                    }
                }
                KeyCode::Esc => return Ok(Outcome::Quit),
                _ => (),
            }
        }

        loop {
            match self.inspector_rx.try_recv() {
                Ok(message) => self.on_inspector_message(message),
                Err(e) => match e {
                    TryRecvError::Disconnected => return Err(Error::InspectorDisconnected),
                    _ => break,
                },
            }
        }

        Ok(Outcome::Success)
    }

    pub fn get_operation_log(&self) -> &[CpuStepResult] {
        self.operation_log.as_slice()
    }

    pub fn get_command_log(&self) -> &[CommandResult] {
        self.command_log.as_slice()
    }

    pub fn push_operation(&mut self, result: CpuStepResult) {
        if self.operation_log.len() >= Self::MAX_OPERATION_LOG_LEN {
            self.operation_log.remove(0);
        }

        self.operation_log.push(result);
    }

    pub fn push_command(&mut self, result: CommandResult) {
        if self.command_log.len() >= Self::MAX_COMMAND_LOG_LEN {
            self.command_log.remove(0);
        }

        self.command_log.push(result);
    }

    fn on_command(&mut self) -> Result<Outcome> {
        let command: Command = if self.input.len() > 0 {
            self.input.parse()?
        } else {
            self.last_command.take().unwrap_or(Command::Next)
        };

        let output = match command {
            Command::Quit => return Ok(Outcome::Quit),
            Command::Reset => return Ok(Outcome::Reset),
            Command::Next => {
                self.hardware.step();
                None
            }
            Command::CartInfo => Some(CommandOutput::CartInfo {
                title: self.hardware.memory.cartridge.title.clone(),
                mbc_kind: self
                    .hardware
                    .memory
                    .cartridge
                    .controller
                    .get_controller_type(),
            }),
            Command::ReadByte(ref inner) => {
                let value = self.hardware.memory.read_byte(inner.address);

                Some(CommandOutput::ReadByte {
                    address: inner.address,
                    value,
                })
            }
            Command::ReadWord(ref inner) => {
                let value = self.hardware.memory.read_word(inner.address);

                Some(CommandOutput::ReadWord {
                    address: inner.address,
                    value,
                })
            }
            Command::WriteByte(ref inner) => {
                self.hardware.memory.write_byte(inner.address, inner.value);

                Some(CommandOutput::WriteByte {
                    address: inner.address,
                    value: inner.value,
                })
            }
            Command::WriteWord(ref inner) => {
                self.hardware.memory.write_word(inner.address, inner.value);

                Some(CommandOutput::WriteWord {
                    address: inner.address,
                    value: inner.value,
                })
            }
        };

        self.input.clear();
        self.last_command = Some(command.clone());
        self.push_command(CommandResult { command, output });

        Ok(Outcome::Success)
    }

    fn on_inspector_message(&mut self, message: Message) {
        if let Message::Operation { pc, op } = message {
            self.push_operation(CpuStepResult {
                operation: op.kind,
                address: pc,
            });
        }
    }
}

#[derive(Debug)]
pub enum Outcome {
    Success,
    Reset,
    Quit,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("hardware error: {0}")]
    Hardware(#[from] gb_rs_core::Error),

    #[error("command error: {0}")]
    Command(#[from] command::Error),

    #[error("cpu inspector disconnected unexpectedly")]
    InspectorDisconnected,
}

#[derive(Debug)]
pub struct CpuStepResult {
    pub operation: OperationKind,
    pub address: u16,
}

#[derive(Debug)]
pub struct CommandResult {
    pub command: Command,
    pub output: Option<CommandOutput>,
}

#[derive(Debug)]
pub enum CommandOutput {
    CartInfo {
        title: String,
        mbc_kind: ControllerType,
    },
    ReadByte {
        address: u16,
        value: u8,
    },
    ReadWord {
        address: u16,
        value: u16,
    },
    WriteByte {
        address: u16,
        value: u8,
    },
    WriteWord {
        address: u16,
        value: u16,
    },
}
