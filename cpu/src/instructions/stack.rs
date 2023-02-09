use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::{containers::Cycles, instructions::stack::*};
use gb_rs_core::bytes::{bytes_to_word, word_to_bytes};
use gb_rs_memory::Memory;

impl Execute for Stack {
    enum_pass_execute!(Self::Push(inner), Self::Pop(inner));
}

impl Execute for PushStack {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use PushStackTarget::*;

        let value = match self.target {
            Pair(pair) => cpu.registers.get_pair(pair),
            AccumulatorAndFlags => bytes_to_word(cpu.registers.a, *cpu.registers.flags),
        };

        cpu.registers.stack_pointer -= 2;
        memory.write_word(cpu.registers.stack_pointer, value);

        cycles.into()
    }
}

impl Execute for PopStack {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = memory.read_word(cpu.registers.stack_pointer);
        cpu.registers.stack_pointer += 2;

        use PopStackTarget::*;

        match self.target {
            Pair(pair) => cpu.registers.set_pair(pair, value),
            AccumulatorAndFlags => {
                let [a, flags] = word_to_bytes(value);

                cpu.registers.a = a;
                cpu.registers.flags = flags.into();
            }
        };

        cycles.into()
    }
}
