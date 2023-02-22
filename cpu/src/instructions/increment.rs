use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::containers::{Cycles, Flag};
use gb_rs_asm::instructions::increment::{
    Increment, PairIncrement, PairPointerIncrement, RegisterIncrement,
};
use gb_rs_common::Z80Add;
use gb_rs_memory::Memory;

impl Execute for Increment {
    enum_pass_execute!(
        Self::Pair(inner),
        Self::Register(inner),
        Self::PairPointer(inner),
    );
}

impl Execute for PairIncrement {
    fn execute(self, cpu: &mut Cpu, _memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = cpu.registers.get_pair(self.target).wrapping_add(1);
        cpu.registers.set_pair(self.target, value);

        cycles.into()
    }
}

impl Execute for RegisterIncrement {
    fn execute(self, cpu: &mut Cpu, _memory: &mut Memory, cycles: Cycles) -> Effect {
        let output = cpu.registers.get_byte(self.target).add_with_flags(1);
        cpu.registers.set_byte(self.target, output.result);

        cpu.registers.flags.unset(Flag::Subtract);
        cpu.registers.flags.set_if(Flag::Zero, output.is_zero());

        cpu.registers
            .flags
            .set_if(Flag::HalfCarry, output.half_carry);

        cycles.into()
    }
}

impl Execute for PairPointerIncrement {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let addr = cpu.registers.get_pair(*self.target);
        let output = memory.read_byte(addr).add_with_flags(1);
        memory.write_byte(addr, output.result);

        cpu.registers.flags.unset(Flag::Subtract);
        cpu.registers.flags.set_if(Flag::Zero, output.is_zero());

        cpu.registers
            .flags
            .set_if(Flag::HalfCarry, output.half_carry);

        cycles.into()
    }
}
