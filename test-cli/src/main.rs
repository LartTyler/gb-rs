use clap::Parser;
use cli::Cli;
use console::{style, StyledObject};
use dialoguer::{theme::ColorfulTheme, Input};
use gb_rs_asm::{containers::Pair, instructions::Instruction, operations::Operation};
use gb_rs_cpu::{inspector::Message, registers::Registers, Cpu};
use gb_rs_memory::Memory;
use std::{fs::File, io::Read, path::Path, sync::mpsc::TryRecvError};

mod cli;

fn main() {
    let cli = Cli::parse();

    let cart = load_file(&cli.cart_file).unwrap();

    let mut memory = Memory::new(cart).unwrap();
    let mut cpu = Cpu::new(memory.cartridge.device_mode.into());

    let mut interact = Interact::new();
    let inspector_rx = cpu.inspect();

    loop {
        let command = match interact.prompt() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        match command {
            Command::Quit => break,
            Command::Info => show_device_state(&cpu, &memory),
            Command::Next => cpu.step(&mut memory),
            Command::Memory(addr, kind) => {
                let value = match kind {
                    MemoryReadKind::Byte => memory.read_byte(addr).into(),
                    MemoryReadKind::Word => memory.read_word(addr),
                };

                println!("{} ${addr:04X} = {value:#06X} ({value:>5})", prefix_style());
            }
        };

        loop {
            match inspector_rx.try_recv() {
                Ok(message) => on_inspector_message(message),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("Inspector disconnected unexpectedly"),
            };
        }
    }
}

fn show_device_state(cpu: &Cpu, memory: &Memory) {
    println!("");
    cpu.show();
    println!("");

    let next_op = memory.read_byte(cpu.registers.program_counter);

    let next_op = match next_op {
        0xCB => cpu
            .instructions
            .extended(memory.read_byte(cpu.registers.program_counter + 1)),
        n => cpu.instructions.base(n),
    };

    println!("{}", title_style("Next:"));
    next_op.unwrap().show();
    println!("");
}

fn on_inspector_message(message: Message) {
    match message {
        Message::Operation(op) => op.show(),

        // Silently ignore messages we don't care about
        _ => (),
    };
}

fn load_file(path: &Path) -> Result<Vec<u8>, LoadError> {
    let mut file = File::open(path)?;

    let len = file.metadata()?.len();
    let len: usize = len.try_into().map_err(|_| LoadError::Size)?;

    let mut data: Vec<u8> = Vec::with_capacity(len);
    file.read_to_end(&mut data)?;

    Ok(data)
}

#[derive(Debug, thiserror::Error)]
enum LoadError {
    #[error("load error: {0}")]
    IO(#[from] std::io::Error),

    #[error("load error: file too big")]
    Size,
}

struct Interact {
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
enum MemoryReadKind {
    Byte,
    Word,
}

#[derive(Debug, Clone, Copy)]
enum Command {
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
enum InteractError {
    #[error("interact error: {0}")]
    IO(#[from] std::io::Error),

    #[error("interact error: empty command")]
    Empty,

    #[error("interact error: unrecognized command")]
    UnrecognizedCommand,
}

trait Show {
    fn show(&self);
}

impl Show for Cpu {
    fn show(&self) {
        println!("{}", title_style("Registers:"));
        self.registers.show();

        println!("interrupts: {}", value_style(self.interrupts_enabled));
    }
}

impl Show for Registers {
    fn show(&self) {
        let Self {
            a,
            b,
            c,
            d,
            e,
            h,
            l,
            flags,
            program_counter,
            stack_pointer,
            ..
        } = self;

        println!("a: {:>3}, flags: {}", value_style(a), value_style(flags));

        println!(
            "b: {:>3}, c: {:>3} || bc: {:>5}",
            value_style(b),
            value_style(c),
            value_style(self.get_pair(Pair::BC)),
        );

        println!(
            "d: {:>3}, e: {:>3} || de: {:>5}",
            value_style(d),
            value_style(e),
            value_style(self.get_pair(Pair::DE))
        );

        println!(
            "h: {:>3}, l: {:>3} || hl: {:>5}",
            value_style(h),
            value_style(l),
            value_style(self.get_pair(Pair::HL))
        );

        let pc = value_style(program_counter);
        let sp = value_style(stack_pointer);
        println!("pc: {pc:#06X} ({pc:>5}), sp: {sp:#06X} ({sp:>5})");
    }
}

impl Show for Operation {
    fn show(&self) {
        println!("{} {}", prefix_style(), self.kind);
    }
}

impl Show for Instruction {
    fn show(&self) {
        let prefix = prefix_style();

        println!("{prefix} {}", self.kind);
        println!("{prefix} w = {}, c = {}", self.width, self.cycles);
    }
}

fn prefix_style() -> StyledObject<&'static str> {
    style(">>").black().bright()
}

fn title_style<'a>(title: &'a str) -> StyledObject<&'a str> {
    style(title).bold()
}

fn value_style<T>(value: T) -> StyledObject<T> {
    style(value).white().dim()
}
