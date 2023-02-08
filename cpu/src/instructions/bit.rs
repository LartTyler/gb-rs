use super::{Effect, Execute};
use crate::Cpu;
use gb_rs_asm::{
    containers::{Cycles, Flag},
    operations::bit::*,
};
use gb_rs_memory::Memory;

impl Execute for Bit {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        match self {
            Self::SetCarryFlag => {
                cpu.registers.flags.set(Flag::Carry);
                cpu.registers.flags.unset(Flag::Subtract);
                cpu.registers.flags.unset(Flag::HalfCarry);

                Effect { cycles: 1 }
            }
            _ => todo!(),
        }
    }
}

impl Execute for BitwiseComplement {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use BitwiseComplementTarget::*;

        let result = match self.target {
            Register(reg) => {
                let value = !cpu.registers.get_byte(reg);
                cpu.registers.set_byte(reg, value);

                value
            }
            Flag(flag) => {
                cpu.registers.flags.toggle(flag);
                cpu.registers.flags.has(flag) as u8;
            }
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                let value = !memory.read_byte(addr);
                memory.write_byte(addr, value);

                value
            }
            Data(data) => {}
        };

        cycles.into()
    }
}
