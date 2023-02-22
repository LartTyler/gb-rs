use gb_rs_asm::{containers::Condition, sets::Instructions};
use gb_rs_common::DeviceMode;
use gb_rs_memory::Memory;
use inspector::{Inspector, Message};
use instructions::{Effect, Execute};
use registers::{FlagsRegister, Registers};
use std::sync::mpsc::Receiver;

pub mod inspector;
pub mod instructions;
pub mod registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub instructions: Instructions,
    pub cycle_counter: u16,
    pub interrupts_enabled: bool,
    inspector: Inspector,
}

impl Cpu {
    pub fn new(mode: DeviceMode) -> Self {
        Self {
            instructions: Instructions::default(),
            cycle_counter: 0,
            interrupts_enabled: true,
            registers: Registers::new(mode),
            inspector: Inspector::new(),
        }
    }

    pub fn inspect(&mut self) -> Receiver<Message> {
        self.inspector.connect()
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let operation = self
            .instructions
            .parse(memory, self.registers.program_counter)
            .unwrap();

        let pc = self.registers.program_counter;

        self.inspector.send_fn(|| Message::Operation {
            op: operation.clone(),
            pc,
        });

        // PC needs to be updated BEFORE executing the instruction, otherwise we end up with
        // inconsistent positioning if the instruction changes PC, e.g. during a relative jump.
        self.registers.update_pc(operation.width);

        let Effect { cycles } = operation.kind.execute(self, memory, operation.cycles);
        self.update_cycles(cycles);

        self.inspector.send(Message::Step);
    }

    fn update_cycles<C>(&mut self, cycles: C)
    where
        C: Into<u16>,
    {
        self.cycle_counter = self.cycle_counter.wrapping_add(cycles.into());
    }
}

pub trait ConditionTest {
    fn test(&self, flags: &FlagsRegister) -> bool;
}

impl ConditionTest for Condition {
    fn test(&self, flags: &FlagsRegister) -> bool {
        use Condition::*;

        match self {
            Always => true,
            Set(flag) => flags.has(*flag),
            Unset(flag) => !flags.has(*flag),
        }
    }
}
