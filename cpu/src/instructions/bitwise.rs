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
/// - **Zero**: Always unset
/// - **Subtract**: Always unset
/// - **Half-Carry**: Always unset
/// - **Carry**: If bit 7 was set before the rotation
pub fn rotate_left(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let (value, carry) = registers.get_byte(reg).overflowing_shl(1);
    registers.set_byte(reg, value + carry as u8);

    registers.clear_flags();
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Same as [`rotate_left()`], except it also sets the [`Flag::Zero`] flag based on the result of
/// the operation.
pub fn rotate_left_extended(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let effect = rotate_left(registers, reg);
    registers.set_flag(Flag::Zero, registers.get_byte(reg) == 0);

    effect
}

/// Implementation of [`rotate_left()`] for A
pub fn rotate_left_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left(registers, ByteRegister::A)
}

/// Implementation of [`rotate_left_extended()`] for A
pub fn rotate_left_a_extended(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::A)
}

/// Implementation of [`rotate_left_extended()`] for B
pub fn rotate_left_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::B)
}

/// Implementation of [`rotate_left_extended()`] for C
pub fn rotate_left_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::C)
}

/// Implementation of [`rotate_left_extended()`] for D
pub fn rotate_left_d(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::D)
}

/// Implementation of [`rotate_left_extended()`] for E
pub fn rotate_left_e(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::E)
}

/// Implementation of [`rotate_left_extended()`] for H
pub fn rotate_left_h(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::H)
}

/// Implementation of [`rotate_left_extended()`] for L
pub fn rotate_left_l(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_extended(registers, ByteRegister::L)
}

/// Performs a wrapping bitwise right shift that also copies bit 0 into the carry flag.
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Zero**: Always unset
/// - **Subtract**: Always unset
/// - **Half-Carry**: Always unset
/// - **Carry**: If bit 0 was set before the rotation
pub fn rotate_right(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let (value, carry) = registers.get_byte(reg).overflowing_shr(1);
    registers.set_byte(reg, value + carry as u8);

    registers.clear_flags();
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Same as [`rotate_right()`], except it also sets the [`Flag::Zero`] flag based on the result of
/// the operation.
pub fn rotate_right_extended(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let effect = rotate_right(registers, reg);
    registers.set_flag(Flag::Zero, registers.get_byte(reg) == 0);

    effect
}

/// Implementation of [`rotate_right()`] for A
pub fn rotate_right_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right(registers, ByteRegister::A)
}

/// Implementation of [`rotate_right_extended()`] for A
pub fn rotate_right_a_extended(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::A)
}

/// Implementation of [`rotate_right_extended()`] for B
pub fn rotate_right_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::B)
}

/// Implementation of [`rotate_right_extended()`] for C
pub fn rotate_right_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::C)
}

/// Implementation of [`rotate_right_extended()`] for D
pub fn rotate_right_d(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::D)
}

/// Implementation of [`rotate_right_extended()`] for E
pub fn rotate_right_e(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::E)
}

/// Implementation of [`rotate_right_extended()`] for H
pub fn rotate_right_h(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::H)
}

/// Implementation of [`rotate_right_extended()`] for L
pub fn rotate_right_l(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_extended(registers, ByteRegister::L)
}
