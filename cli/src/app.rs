use crate::command::Command;
use gb_rs_asm::operations::OperationKind;
use gb_rs_core::{cpu::inspector::Message, Hardware};
use std::{path::Path, sync::mpsc::Receiver};

pub type Result<T> = std::result::Result<T, Error>;

pub struct App {
    pub last_command: Command,
    pub hardware: Hardware,
    pub operation_log: Vec<String>,
    pub command_log: Vec<String>,
    inspector_rx: Receiver<Message>,
}

impl App {
    const MAX_OPERATION_LOG_LEN: usize = 50;
    const MAX_COMMAND_LOG_LEN: usize = 50;

    pub fn run(&mut self) -> Result<Outcome> {
        Ok(Outcome::Success)
    }

    pub fn from_file(cart_file: &Path) -> Result<Self> {
        let mut hardware = Hardware::from_file(cart_file)?;

        Ok(Self {
            inspector_rx: hardware.cpu.inspect(),
            hardware: Hardware::from_file(cart_file)?,
            last_command: Command::Next,
            operation_log: Vec::with_capacity(Self::MAX_OPERATION_LOG_LEN),
            command_log: Vec::with_capacity(Self::MAX_COMMAND_LOG_LEN),
        })
    }

    pub fn push_operation(&mut self, op: OperationKind, address: u16) {
        if self.operation_log.len() >= Self::MAX_OPERATION_LOG_LEN {
            self.operation_log.remove(0);
        }

        self.operation_log.push(format!("${address:04X} | {op}"));
    }

    pub fn push_command(&mut self, command: Command, output_lines: &[String]) {
        if self.command_log.len() >= Self::MAX_COMMAND_LOG_LEN {
            self.command_log.remove(0);
        }

        self.command_log.push(format!("{command}"));
    }
}

#[derive(Debug)]
pub enum Outcome {
    Success,
    Reset,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("hardware error: {0}")]
    Hardware(#[from] gb_rs_core::Error),
}

pub struct CommandResult {
    command: Command,
    output: CommandOutput,
}
