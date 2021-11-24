use crate::instructions::get_instruction;
use crate::registers::Registers;
use gb_rs_memory::Memory;

pub mod instructions;
pub mod registers;

pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn step(&mut self, memory: &mut Memory) {
        let opcode = memory.read_byte(self.registers.stack_pointer);
        let instruction = get_instruction(opcode);

        if instruction.is_none() {
            panic!(
                "Unrecognized instruction opcode {:#x} at {:#x}",
                opcode, self.registers.stack_pointer
            );
        }

        let effect = instruction.unwrap()(&mut self.registers, memory);
        self.registers.stack_pointer += effect.width_bytes as u16;
    }
}
