use crate::instructions::Effect;
use crate::registers::{Flag, Registers};
use gb_rs_memory::Memory;

/// Adjusts the accumulator (A) register to convert the result of an operation back to a binary
/// coded decimal (BCD).
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Zero**: If the result was zero
/// - **Half-Carry**: Always unset
/// - **Carry**: If there was a carry out of the high BCD digit (high nibble > 9)
pub fn decimal_adjust_accum(registers: &mut Registers, _: &mut Memory) -> Effect {
    let mut adjust = 0;
    let value = registers.a;

    if registers.get_flag(Flag::HalfCarry) || (value & 0xF) > 9 {
        adjust = 0x06;
    }

    if registers.get_flag(Flag::Carry) || (value & 0xF0) > 9 {
        adjust |= 0x60;
    }

    let value = if registers.get_flag(Flag::Subtract) {
        value.wrapping_sub(adjust)
    } else {
        value.wrapping_add(adjust)
    };

    registers.a = value;

    registers.set_flag(Flag::Zero, value == 0);
    registers.set_flag(Flag::HalfCarry, false);
    registers.set_flag(Flag::Carry, adjust & 0x60 != 0);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}
