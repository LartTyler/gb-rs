use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::{
    containers::{Cycles, Flag, Pair},
    operations::add::*,
};
use gb_rs_core::Z80Add;
use gb_rs_memory::Memory;

impl Execute for Add {
    enum_pass_execute!(Self::Pair(inner), Self::Register(inner));
}

impl Execute for PairAdd {
    fn execute(self, cpu: &mut Cpu, _memory: &mut Memory, cycles: Cycles) -> Effect {
        let lhs = cpu.registers.get_pair(self.target);

        let output = match self.source {
            PairAddSource::Pair(source) => {
                let rhs = cpu.registers.get_pair(source);
                lhs.add_with_flags(rhs)
            }
            PairAddSource::SignedData(data) => lhs.add_with_flags(data.as_twos_complement()),
        };

        cpu.registers.set_pair(self.target, output.result);

        cpu.registers.flags.unset(Flag::Subtract);
        cpu.registers.flags.update_from_math_result(&output);

        if self.target == Pair::SP {
            cpu.registers.flags.unset(Flag::Zero);
        }

        cycles.into()
    }
}

impl Execute for RegisterAdd {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use RegisterAddSource::*;

        let rhs = match self.source {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
            Data(data) => *data,
        };

        let output = cpu.registers.a.add_with_flags(rhs);
        let output = if self.with_carry {
            let carry = cpu.registers.flags.has(Flag::Carry) as u8;
            let carry_output = output.result.add_with_flags(carry);

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
