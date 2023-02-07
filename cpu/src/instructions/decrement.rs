use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::containers::{Cycles, Flag, Pair};
use gb_rs_asm::instructions::decrement::{
    Decrement, PairDecrement, PairPointerDecrement, RegisterDecrement,
};
use gb_rs_core::Z80Sub;
use gb_rs_memory::Memory;

impl Execute for Decrement {
    enum_pass_execute!(
        Self::Register(inner),
        Self::Pair(inner),
        Self::PairPointer(inner),
    );
}

impl Execute for RegisterDecrement {
    fn execute(self, cpu: &mut Cpu, _memory: &mut Memory, cycles: Cycles) -> Effect {
        let output = cpu.registers.get_byte(self.target).sub_with_flags(1);
        cpu.registers.set_byte(self.target, output.result);

        cpu.registers.flags.reset();
        cpu.registers.flags.set(Flag::Subtract);

        cpu.registers
            .flags
            .set_if(Flag::HalfCarry, output.half_carry);

        cycles.into()
    }
}

impl Execute for PairDecrement {
    fn execute(self, cpu: &mut Cpu, _memory: &mut Memory, cycles: Cycles) -> Effect {
        let output = cpu.registers.get_pair(self.target).sub_with_flags(1);
        cpu.registers.set_pair(self.target, output.result);

        cpu.registers.flags.reset();

        if self.target != Pair::SP {
            cpu.registers.flags.set(Flag::Subtract);

            cpu.registers
                .flags
                .set_if(Flag::HalfCarry, output.half_carry);
        }

        cycles.into()
    }
}

impl Execute for PairPointerDecrement {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let addr = cpu.registers.get_pair(*self.target);
        let output = memory.read_byte(addr).sub_with_flags(1);

        cpu.registers.flags.reset();
        cpu.registers.flags.set(Flag::Subtract);

        cpu.registers
            .flags
            .set_if(Flag::HalfCarry, output.half_carry);

        cycles.into()
    }
}
