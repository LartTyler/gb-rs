use super::{Effect, Execute};
use crate::{enum_pass_execute, ConditionTest, Cpu};
use gb_rs_asm::{containers::Cycles, operations::jump::*};
use gb_rs_memory::Memory;

impl Execute for Jump {
    enum_pass_execute!(Self::Absolute(inner), Self::Relative(inner));
}

impl Execute for AbsoluteJump {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use AbsoluteJumpTarget::*;

        let addr = match self.target {
            PairPointer(pointer) => cpu.registers.get_pair(*pointer),
            DataPointer(pointer) => **pointer,
        };

        let jumped = if self.condition.test(&cpu.registers.flags) {
            cpu.registers.program_counter = addr;

            true
        } else {
            false
        };

        Effect {
            cycles: cycles.with_toggle(jumped),
        }
    }
}

impl Execute for RelativeJump {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let jumped = if self.condition.test(&cpu.registers.flags) {
            let addr = cpu.registers.program_counter;
            let addr = addr.wrapping_add_signed(self.offset.into());
            cpu.registers.program_counter = addr;

            true
        } else {
            false
        };

        Effect {
            cycles: cycles.with_toggle(jumped),
        }
    }
}
