use super::{Effect, Execute};
use crate::{enum_pass_execute, ConditionTest, Cpu};
use gb_rs_asm::{
    containers::Cycles,
    instructions::subroutine::Return,
    operations::subroutine::{Call, Subroutine},
};
use gb_rs_memory::Memory;

impl Execute for Subroutine {
    enum_pass_execute!(Self::Return(inner), Self::Call(inner));
}

impl Execute for Return {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let branched = if self.condition.test(&cpu.registers.flags) {
            let target_addr = memory.read_word(cpu.registers.stack_pointer);
            cpu.registers.stack_pointer += 2;

            if self.enable_interrupt {
                cpu.interrupts_enabled = true;
            }

            cpu.registers.program_counter = target_addr;

            true
        } else {
            false
        };

        Effect {
            cycles: cycles.with_toggle(branched),
        }
    }
}

impl Execute for Call {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let branched = if self.condition.test(&cpu.registers.flags) {
            cpu.registers.stack_pointer -= 2;

            memory.write_word(
                cpu.registers.stack_pointer,
                // CALL instructions are 3 bytes wide (including the opcode), and RET expects the
                // return address on the stack to be the instruction AFTER the CALL instruction
                // that is being returned from.
                cpu.registers.program_counter + 3,
            );

            cpu.registers.stack_pointer -= 2;

            let target_addr = memory.read_word(cpu.registers.program_counter);
            cpu.registers.program_counter = target_addr;

            true
        } else {
            false
        };

        Effect {
            cycles: cycles.with_toggle(branched),
        }
    }
}
