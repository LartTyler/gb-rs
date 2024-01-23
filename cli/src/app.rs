use crate::{
    auto::{start_auto_tick, AutoTick},
    command::{self, Command},
};
use crossterm::event::{self, Event, KeyCode};
use gb_rs_asm::operations::OperationKind;
use gb_rs_core::{cpu::inspector::Message, memory::cartridge::mbc::ControllerType, Hardware};
use std::{
    path::Path,
    sync::mpsc::{Receiver, TryRecvError},
    time::Duration,
};

pub type Result<T> = std::result::Result<T, Error>;

pub struct App {
    pub auto_tick: Option<AutoTick>,
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
            auto_tick: None,
        })
    }

    pub fn run(&mut self) -> Result<Outcome> {
        if let Some(AutoTick { receiver, .. }) = &self.auto_tick {
            loop {
                match receiver.try_recv() {
                    Ok(_) => self.hardware.step(),
                    Err(e) => match e {
                        TryRecvError::Disconnected => return Err(Error::AutoTick),
                        _ => break,
                    },
                }
            }
        }

        if let Some(outcome) = self.handle_input()? {
            return Ok(outcome);
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

    fn handle_input(&mut self) -> Result<Option<Outcome>> {
        if !event::poll(Duration::from_millis(10))? {
            return Ok(None);
        }

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
                        return Ok(Some(outcome));
                    }
                }
                KeyCode::Esc => {
                    // ESC only triggers a quit if we're not currently auto-ticking. Otherwise,
                    // it should just cancel the auto tick.
                    if !self.stop_auto_tick()? {
                        return Ok(Some(Outcome::Quit));
                    }
                }
                _ => (),
            }
        }

        Ok(None)
    }

    fn on_command(&mut self) -> Result<Outcome> {
        let command: Command = if !self.input.is_empty() {
            self.input.parse()?
        } else {
            self.last_command.clone().unwrap_or(Command::Next)
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
            Command::Auto(ref inner) => {
                self.auto_tick = Some(start_auto_tick(inner.tick_rate));

                Some(CommandOutput::Auto {
                    info: format!("Ticking system at {:#}s", inner.tick_rate.as_secs_f64()),
                })
            }
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

        if Self::is_history_allowed(&command) {
            self.last_command = Some(command.clone());
        }

        self.input.clear();
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

    fn is_history_allowed(command: &Command) -> bool {
        matches!(command, Command::Next)
    }

    fn stop_auto_tick(&mut self) -> Result<bool> {
        match self.auto_tick.take() {
            Some(auto_tick) => {
                // We need to explicitly drop the receiver before attempting to join the tick
                // thread, since the trigger for the thread's loop to terminate is the receiver
                // hanging up.
                drop(auto_tick.receiver);

                if auto_tick.join_handle.join().is_err() {
                    return Err(Error::AutoTick);
                }

                Ok(true)
            }
            _ => Ok(false),
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

    #[error("auto tick thread completed with an error")]
    AutoTick,
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
    Auto {
        info: String,
    },
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
