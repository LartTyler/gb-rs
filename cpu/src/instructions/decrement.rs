use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, PairRegister, Registers};
use gb_rs_core::bytes::is_half_carry_sub;
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
    registers.set_flag(Flag::HalfCarry, is_half_carry_sub(value, 1));

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Implementation of [`decrement_byte_register()`] for A
pub fn decrement_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::A)
}

/// Implementation of [`decrement_byte_register()`] for B
pub fn decrement_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::B)
}

/// Implementation of [`decrement_byte_register()`] for C
pub fn decrement_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::C)
}

/// Implementation of [`decrement_byte_register()`] for D
pub fn decrement_d(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::D)
}

/// Implementation of [`decrement_byte_register()`] for E
pub fn decrement_e(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::E)
}

/// Implementation of [`decrement_byte_register()`] for H
pub fn decrement_h(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::H)
}

/// Implementation of [`decrement_byte_register()`] for L
pub fn decrement_l(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_byte_register(registers, ByteRegister::L)
}

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

/// Implementation of [`decrement_pair()`] for BC
pub fn decrement_bc(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_pair(registers, PairRegister::BC)
}

/// Implementation of [`decrement_pair()`] for DE
pub fn decrement_de(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_pair(registers, PairRegister::DE)
}

/// Implementation of [`decrement_pair()`] for HL
pub fn decrement_hl(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_pair(registers, PairRegister::HL)
}

/// Implementation of [`decrement_pair()`] for SP
pub fn decrement_sp(registers: &mut Registers, _: &mut Memory) -> Effect {
    decrement_pair(registers, PairRegister::SP)
}
