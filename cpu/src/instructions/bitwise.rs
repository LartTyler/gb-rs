use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, Registers};
use gb_rs_memory::Memory;

/// Performs a wrapping bitwise left shift that also copies bit 7 into the carry flag.
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Zero**: If value is zero (except for register A, which is always unset)
/// - **Subtract**: Always unset
/// - **Half-Carry**: Always unset
/// - **Carry**: If bit 7 was set before the rotation
pub fn rotate_left(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let (value, carry) = registers.get_byte(reg).overflowing_shl(1);
    registers.set_byte(reg, value + carry as u8);

    registers.clear_flags();
    registers.set_flag(Flag::Carry, carry);

    if reg != ByteRegister::A {
        registers.set_flag(Flag::Zero, value == 0);
    }

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Implementation of [`rotate_left()`] for A
pub fn rotate_left_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::A)
}

/// Implementation of [`rotate_left()`] for B
pub fn rotate_left_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::B)
}

/// Implementation of [`rotate_left()`] for C
pub fn rotate_left_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::C)
}

/// Implementation of [`rotate_left()`] for D
pub fn rotate_left_d(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::D)
}

/// Implementation of [`rotate_left()`] for E
pub fn rotate_left_e(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::E)
}

/// Implementation of [`rotate_left()`] for H
pub fn rotate_left_h(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::H)
}

/// Implementation of [`rotate_left()`] for L
pub fn rotate_left_l(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::L)
}
