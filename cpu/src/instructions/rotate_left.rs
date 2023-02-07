use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::containers::{Cycles, Flag};
use gb_rs_asm::instructions::rotate_left::{
    CarryingRotateLeft, CarryingRotateLeftTarget, CyclicRotateLeft, CyclicRotateLeftTarget,
    RotateLeft,
};
use gb_rs_memory::Memory;

impl Execute for RotateLeft {
    enum_pass_execute!(Self::Cylic(inner), Self::Carrying(inner));
}

impl Execute for CyclicRotateLeft {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use CyclicRotateLeftTarget::*;

        let value = match self.target {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let value = value.rotate_left(1);

        cpu.registers.flags.reset();
        cpu.registers.flags.set_if(Flag::Carry, value & 1 > 0);

        if self.extended {
            cpu.registers.flags.set_if(Flag::Zero, value == 0);
        }

        match self.target {
            Register(reg) => cpu.registers.set_byte(reg, value),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.write_byte(addr, value);
            }
        };

        cycles.into()
    }
}

impl Execute for CarryingRotateLeft {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use CarryingRotateLeftTarget::*;

        let value = match self.target {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let (value, new_carry) = value.overflowing_shl(1);
        let value = value | cpu.registers.flags.has(Flag::Carry) as u8;

        cpu.registers.flags.reset();
        cpu.registers.flags.set_if(Flag::Carry, new_carry);

        if self.extended {
            cpu.registers.flags.set_if(Flag::Zero, value == 0);
        }

        match self.target {
            Register(reg) => cpu.registers.set_byte(reg, value),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.write_byte(addr, value);
            }
        };

        cycles.into()
    }
}
