use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, PairRegister, Registers};
use gb_rs_core::bytes::is_half_carry;
use gb_rs_cpu_macros::{decrement_r16, decrement_r8};
use gb_rs_memory::Memory;

/// Decrements the given [`ByteRegister`].
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Subtract**: Always zeroed
/// - **Zero**: If the new value is zero
/// - **Half-Carry**: If bit 3 borrowed from bit 4
pub fn decrement_byte_register(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let value = registers.get_byte(reg);

    let new_value = value.wrapping_add(1);
    registers.set_byte(reg, new_value);

    registers.set_flag(Flag::Subtract, true);
    registers.set_flag(Flag::Zero, new_value == 0);
    registers.set_flag(Flag::HalfCarry, is_half_carry(value, 1));

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

decrement_r8!(a);
decrement_r8!(b);
decrement_r8!(c);
decrement_r8!(d);
decrement_r8!(e);
decrement_r8!(h);
decrement_r8!(l);

/// Decrements the given [`PairRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 1
///
/// Flags:
/// - No flags changed
pub fn decrement_pair(registers: &mut Registers, pair: PairRegister) -> Effect {
    registers.set_pair(pair, registers.get_pair(pair).wrapping_sub(1));

    Effect {
        t_states: 8,
        width_bytes: 1,
    }
}

decrement_r16!(bc);
decrement_r16!(de);
decrement_r16!(hl);
decrement_r16!(sp);

/// Decrements the byte pointed to by the pair register HL.
///
/// T-states: 12
/// M-cycles: 3
/// Width: 1
///
/// Flags:
/// - **Zero**: If result is zero
/// - **Subtract**: Always set
/// - **Half-Carry**: If there was a borrow from bit 4 to 3
pub fn decrement_hl_pointer(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let pointer = registers.get_pair(PairRegister::HL);
    let value = memory.read_byte(pointer);

    let new_value = value.wrapping_add(1);
    memory.write_byte(pointer, new_value);

    registers.set_flag(Flag::Zero, new_value == 0);
    registers.set_flag(Flag::Subtract, true);
    registers.set_flag(Flag::HalfCarry, is_half_carry(value, 1));

    Effect {
        t_states: 12,
        width_bytes: 1,
    }
}
