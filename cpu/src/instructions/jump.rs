use crate::instructions::Effect;
use crate::registers::{Flag, Registers};
use gb_rs_memory::Memory;

struct Condition(Flag, bool);

impl Condition {
    pub fn evaluate(&self, registers: &Registers) -> bool {
        registers.get_flag(self.0) == self.1
    }
}

/// Performs a relative jump using an immediate signed 8-bit integer.
///
/// The origin of the jump is always the instruction following the jump instruction (e.g. sp + 2).
///
/// T-states: 12
/// M-cycles: 3
/// Width: 2
///
/// Flags:
/// - No flags changed
pub fn jump_relative_immediate_signed(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let origin = registers.stack_pointer + 2; // jump origin is the byte immediately after instruction
    let dest = origin.wrapping_add(memory.read_byte(registers.stack_pointer + 1) as u16);

    registers.stack_pointer = dest;

    Effect {
        t_states: 12,
        width_bytes: 2,
    }
}

/// Performs a relative jump if a given condition is met.
///
/// T-states: 12 (taken) / 8 (untaken)
/// M-cycles: 3 / 2
/// Width: 2
///
/// Flags:
/// - No flags changed
fn jump_relative_conditional(
    registers: &mut Registers,
    memory: &mut Memory,
    cond: Condition,
) -> Effect {
    if cond.evaluate(registers) {
        jump_relative_immediate_signed(registers, memory)
    } else {
        Effect {
            t_states: 8,
            width_bytes: 2,
        }
    }
}

/// Implementation of [`jump_relative_conditional()`] for NZ
pub fn jump_relative_zero_unset(registers: &mut Registers, memory: &mut Memory) -> Effect {
    jump_relative_conditional(registers, memory, Condition(Flag::Zero, false))
}

/// Implementation of [`jump_relative_conditional()`] for Z
pub fn jump_relative_zero_set(registers: &mut Registers, memory: &mut Memory) -> Effect {
    jump_relative_conditional(registers, memory, Condition(Flag::Zero, true))
}

/// Implementation of [`jump_relative_conditional()`] for NC
pub fn jump_relative_carry_unset(registers: &mut Registers, memory: &mut Memory) -> Effect {
    jump_relative_conditional(registers, memory, Condition(Flag::Carry, false))
}

/// Implementation of [`jump_relative_conditional()`] for C
pub fn jump_relative_carry_set(registers: &mut Registers, memory: &mut Memory) -> Effect {
    jump_relative_conditional(registers, memory, Condition(Flag::Carry, true))
}
