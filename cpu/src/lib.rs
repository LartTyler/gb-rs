use gb_rs_asm::sets::Instructions;
use gb_rs_memory::Memory;
use instructions::{Effect, Execute};

pub mod instructions;
pub mod registers;

pub struct Cpu {
    registers: registers::Registers,
    instructions: Instructions,
    cycle_counter: u16,
}

impl Cpu {
    pub fn step(&mut self, memory: &mut Memory) {
        let operation = self
            .instructions
            .parse(memory, self.registers.program_counter)
            .unwrap();

        let Effect { cycles } = operation.kind.execute(self, memory, operation.cycles);

        self.registers.update_pc(operation.width);
        self.update_cycles(cycles);
    }

    fn update_cycles<C>(&mut self, cycles: C)
    where
        C: Into<u16>,
    {
        self.cycle_counter = self.cycle_counter.wrapping_add(cycles.into());
    }
}
