use clap::Parser;
use cli::Cli;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gb_rs_asm::operations::OperationKind;
use gb_rs_cpu::inspector::Message;
use gb_rs_memory::cartridge::mbc::ControllerType;
use hardware::Hardware;
use interact::{Command, ReadKind, WriteKind};
use std::{collections::VecDeque, path::Path, sync::mpsc::TryRecvError};
use tui::{
    backend::Backend,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::ListItem,
    Terminal,
};

mod cli;
mod hardware;
mod interact;
mod ui;

fn main() {
    let cli = Cli::parse();
    let mut terminal = ui::create().unwrap();

    enable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), EnterAlternateScreen).unwrap();

    loop {
        match run_app(&mut terminal, &cli.cart_file) {
            Err(Error::Resetting) => continue,
            Ok(()) => break,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, cart_file: &Path) -> Result<(), Error> {
    let hardware = Hardware::from_file(cart_file).unwrap();
    let app = App::new(hardware);
    app.run(terminal)
}

pub struct App {
    pub hardware: Hardware,
    pub input: String,
    pub instruction_history: VecDeque<String>,
    pub messages: VecDeque<CommandMessage>,
    pub previous_command: Option<Command>,
    quit: bool,
}

const HISTORY_CAPACITY: usize = 50;
const MESSAGE_CAPACITY: usize = 50;

impl App {
    pub fn new(hardware: Hardware) -> Self {
        Self {
            hardware,
            input: String::with_capacity(16),
            instruction_history: VecDeque::with_capacity(HISTORY_CAPACITY),
            messages: VecDeque::with_capacity(MESSAGE_CAPACITY),
            previous_command: None,
            quit: false,
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Error> {
        let inspector_rx = self.hardware.cpu.inspect();

        while !self.quit {
            terminal.draw(|f| ui::layout(f, &self))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => self.input.push(c),
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    KeyCode::Enter => self.on_command_send()?,
                    KeyCode::Esc => break,
                    _ => (),
                };
            }

            loop {
                match inspector_rx.try_recv() {
                    Ok(message) => self.on_inspector_received(message),
                    Err(e) => match e {
                        TryRecvError::Disconnected => return Err(Error::InspectorDisconnected),
                        _ => break,
                    },
                };
            }
        }

        Ok(())
    }

    fn on_command_send(&mut self) -> Result<(), Error> {
        let command: Command = if self.input.len() > 0 {
            let parts: Vec<_> = self.input.split_whitespace().rev().collect();
            parts.try_into()?
        } else {
            self.previous_command.clone().unwrap_or(Command::Next)
        };

        match command {
            Command::Quit => self.quit = true,
            Command::Reset => return Err(Error::Resetting),
            Command::Next => {
                self.hardware.cpu.step(&mut self.hardware.memory);
            }
            Command::CartInfo => {
                let cart = &self.hardware.memory.cartridge;

                self.push_command(
                    command,
                    CommandOutput::CartInfo {
                        title: cart.title.clone(),
                        controller_type: cart.controller.get_controller_type(),
                    },
                );
            }
            Command::Read(address, kind) => {
                let value = match kind {
                    ReadKind::Byte => self.hardware.memory.read_byte(address).into(),
                    ReadKind::Word => self.hardware.memory.read_word(address),
                };

                self.push_command(command, CommandOutput::Read { address, value });
            }
            Command::Write(address, ref kind) => {
                match kind {
                    WriteKind::Byte(n) => self.hardware.memory.write_byte(address, *n),
                    WriteKind::Word(n) => self.hardware.memory.write_word(address, *n),
                };

                let output = CommandOutput::Write {
                    address,
                    value: kind.clone(),
                };

                self.push_command(command, output);
            }
            Command::Convert {
                ref original,
                value,
                format,
            } => {
                let original = original.clone();
                let value = format.as_formatted(value);

                self.push_command(command, CommandOutput::Convert { original, value });
            }
        };

        self.input.clear();

        Ok(())
    }

    fn on_inspector_received(&mut self, message: Message) {
        match message {
            Message::Operation { pc, op } => {
                if self.instruction_history.len() >= HISTORY_CAPACITY {
                    self.instruction_history.pop_front();
                }

                self.instruction_history
                    .push_back(format!("${pc:04X} | {}", op.kind));

                self.push_command(Command::Next, CommandOutput::Next { op: op.kind });
            }

            // Ignore messages we don't care about handling at the moment.
            _ => (),
        };
    }

    fn push_command(&mut self, command: Command, output: CommandOutput) {
        if self.messages.len() >= MESSAGE_CAPACITY {
            self.messages.pop_front();
        }

        self.messages.push_back(CommandMessage {
            command: format!("{command}"),
            output,
        });
    }
}

#[derive(Debug)]
pub struct CommandMessage {
    pub command: String,
    pub output: CommandOutput,
}

#[derive(Debug)]
pub enum CommandOutput {
    CartInfo {
        title: String,
        controller_type: ControllerType,
    },
    Next {
        op: OperationKind,
    },
    Read {
        address: u16,
        value: u16,
    },
    Write {
        address: u16,
        value: WriteKind,
    },
    Convert {
        original: String,
        value: String,
    },
}

impl CommandMessage {
    pub fn as_list_items(&self) -> Vec<ListItem<'_>> {
        let mut items = vec![ListItem::new(Spans::from(vec![
            Span::raw(">> "),
            Span::styled(&self.command, Style::default().fg(Color::DarkGray)),
        ]))];

        match &self.output {
            CommandOutput::Next { op } => {
                let content = Spans::from(vec![
                    Span::raw("Executed "),
                    Span::styled(format!("{op}"), Style::default().fg(Color::DarkGray)),
                ]);

                items.push(ListItem::new(content));
            }
            CommandOutput::CartInfo {
                title,
                controller_type,
            } => {
                let title = Spans::from(vec![
                    Span::raw("Title: "),
                    Span::styled(title, Style::default().fg(Color::DarkGray)),
                ]);

                items.push(ListItem::new(title));

                let controller_type = Spans::from(vec![
                    Span::raw("Controller: "),
                    Span::styled(
                        format!("{controller_type}"),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]);

                items.push(ListItem::new(controller_type));
            }
            CommandOutput::Read { address, value } => {
                items.push(ListItem::new(format!("${address:04X} = {value}")));
            }
            CommandOutput::Write { address, value } => {
                items.push(ListItem::new(format!("set ${address:04X} = {value}")));
            }
            CommandOutput::Convert { original, value } => {
                items.push(ListItem::new(format!("{original} = {value}")));
            }
        };

        items.push(ListItem::new(" "));
        items
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("inspector disconnected unexpectedly")]
    InspectorDisconnected,

    #[error("parse error: {0}")]
    Command(#[from] interact::Error),

    #[error("reset requested")]
    Resetting,
}
