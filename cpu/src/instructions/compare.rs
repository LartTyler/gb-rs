use super::{Effect, Execute};
use crate::Cpu;
use gb_rs_asm::containers::{Cycles, Flag};
use gb_rs_asm::operations::compare::*;
use gb_rs_core::Z80Sub;
use gb_rs_memory::Memory;

impl Execute for Compare {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use CompareTarget::*;

        let rhs = match self.target {
            Register(reg) => cpu.registers.get_byte(reg),
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
            Data(data) => *data,
        };

        let output = cpu.registers.a.sub_with_flags(rhs);

        cpu.registers.flags.update_from_math_result(&output);
        cpu.registers.flags.set(Flag::Subtract);
        cpu.registers.flags.set_if(Flag::Zero, output.result == 0);

        cycles.into()
    }
}
