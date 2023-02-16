use crate::{interact::MemoryReadKind, output::Show};
use clap::Parser;
use cli::Cli;
use gb_rs_cpu::inspector::Message;
use hardware::Hardware;
use interact::{Command, Interact};
use output::prefix_style;
use std::sync::mpsc::TryRecvError;

mod cli;
mod hardware;
mod interact;
mod output;

fn main() {
    let cli = Cli::parse();
    let mut hardware = Hardware::from_file(&cli.cart_file).unwrap();

    let mut interact = Interact::new();
    let inspector_rx = hardware.cpu.inspect();

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
            Command::Info => show_device_state(&hardware),
            Command::Next => hardware.cpu.step(&mut hardware.memory),
            Command::Memory(addr, kind) => {
                let value = match kind {
                    MemoryReadKind::Byte => hardware.memory.read_byte(addr).into(),
                    MemoryReadKind::Word => hardware.memory.read_word(addr),
                };

                println!(
                    "{} ${addr:04X} = {value:#06X} ({value:>5})",
                    output::prefix_style()
                );
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

fn show_device_state(Hardware { cpu, memory }: &Hardware) {
    cpu.show();
    println!("");

    let next_op = memory.read_byte(cpu.registers.program_counter);

    let next_op = match next_op {
        0xCB => cpu
            .instructions
            .extended(memory.read_byte(cpu.registers.program_counter + 1)),
        n => cpu.instructions.base(n),
    };

    println!("{}", output::title_style("Next:"));
    next_op.unwrap().show();
    println!("");
}

fn on_inspector_message(message: Message) {
    match message {
        Message::Operation { op, pc } => {
            println!("{} ${:04X} | {}", prefix_style(), pc, op.kind);
        }

        // Silently ignore messages we don't care about
        _ => (),
    };
}
