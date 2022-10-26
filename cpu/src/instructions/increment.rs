use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, PairRegister, Registers};
use gb_rs_core::bytes::is_half_carry;
use gb_rs_cpu_macros::{increment_r16, increment_r8};
use gb_rs_memory::Memory;

/// Increments the given [`PairRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 1
///
/// Flags:
/// - No flags changed
pub fn increment_pair(registers: &mut Registers, pair: PairRegister) -> Effect {
    let value = registers.get_pair(pair).wrapping_add(1);
    registers.set_pair(pair, value);

    Effect {
        t_states: 8,
        width_bytes: 1,
    }
}

increment_r16!(bc);
increment_r16!(de);
increment_r16!(hl);
increment_r16!(sp);

/// Increments the given [`ByteRegister`].
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Subtract**: Always zeroed
/// - **Zero**: If the new value is zero
/// - **Half-Carry**: If bit 3 carried into bit 4
pub fn increment_byte_register(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let value = registers.get_byte(reg);
    let new_value = value.wrapping_add(1);
    registers.set_byte(reg, new_value);

    registers.set_flag(Flag::Subtract, false);
    registers.set_flag(Flag::Zero, new_value == 0);
    registers.set_flag(Flag::HalfCarry, is_half_carry(value, 1));

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

increment_r8!(a);
increment_r8!(b);
increment_r8!(c);
increment_r8!(d);
increment_r8!(e);
increment_r8!(h);
increment_r8!(l);

/// Increments the byte pointed to by the pair register HL.
///
/// T-states: 12
/// M-cycles: 3
/// Width: 1
///
/// Flags:
/// - **Zero**: If result is zero
/// - **Subtract**: Always reset
/// - **Half-Carry**: If overflow from bit 3 to 4
pub fn increment_hl_pointer(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let pointer = registers.get_pair(PairRegister::HL);
    let value = memory.read_byte(pointer);

    let new_value = memory.read_byte(pointer).wrapping_add(1);
    memory.write_byte(pointer, new_value);

    registers.set_flag(Flag::Zero, new_value == 0);
    registers.set_flag(Flag::Subtract, false);
    registers.set_flag(Flag::HalfCarry, is_half_carry(value, 1));

    Effect {
        t_states: 12,
        width_bytes: 1,
    }
}
