use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::containers::{Cycles, Flag};
use gb_rs_asm::instructions::rotate_right::*;
use gb_rs_memory::Memory;

impl Execute for RotateRight {
    enum_pass_execute!(Self::Cyclic(inner), Self::Carrying(inner));
}

impl Execute for CyclicRotateRight {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use CyclicRotateRightTarget::*;

        let value = match self.target {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let value = value.rotate_right(1);

        cpu.registers.flags.reset();

        // Mask off all bit the most-significant bit. If the result isn't zero, we carried during
        // the previous [`u8::rotate_right()`].
        cpu.registers.flags.set_if(Flag::Carry, value & 0x80 > 0);

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

impl Execute for CarryingRotateRight {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use CarryingRotateRightTarget::*;

        let value = match self.target {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let (value, new_carry) = value.overflowing_shr(1);
        let value = if cpu.registers.flags.has(Flag::Carry) {
            value | 0x80
        } else {
            value
        };

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
        }

        cycles.into()
    }
}
