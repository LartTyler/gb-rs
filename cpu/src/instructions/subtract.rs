use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::{
    containers::{Cycles, Flag},
    operations::subtract::*,
};
use gb_rs_core::Z80Sub;
use gb_rs_memory::Memory;

impl Execute for Subtract {
    enum_pass_execute!(Self::Register(inner));
}

impl Execute for RegisterSubtract {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use RegisterSubtractSource::*;

        let rhs = match self.source {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
            Data(data) => *data,
        };

        let output = cpu.registers.a.sub_with_flags(rhs);
        let output = if self.with_carry {
            let carry = cpu.registers.flags.has(Flag::Carry) as u8;
            let carry_output = output.result.sub_with_flags(carry);

            output.merge(carry_output)
        } else {
            output
        };

        cpu.registers.flags.reset();
        cpu.registers.flags.update_from_math_result(&output);
        cpu.registers.flags.set_if(Flag::Zero, output.result == 0);

        cpu.registers.a = output.result;

        cycles.into()
    }
}
