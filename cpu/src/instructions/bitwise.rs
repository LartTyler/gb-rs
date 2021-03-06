use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, Registers};
use gb_rs_cpu_macros::{
    rotate_left_carry_extended, rotate_left_extended, rotate_right_carry_extended,
    rotate_right_extended,
};
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
    let value = registers.get_byte(reg);
    let carry = value & 0x80 != 0;

    registers.set_byte(reg, (value << 1) | carry as u8);

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

rotate_left_extended!(a);
rotate_left_extended!(b);
rotate_left_extended!(c);
rotate_left_extended!(d);
rotate_left_extended!(e);
rotate_left_extended!(h);
rotate_left_extended!(l);

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
    let value = registers.get_byte(reg);
    let carry = value & 0x80 != 0;

    registers.set_byte(reg, (value >> 1) | carry as u8);

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

rotate_right_extended!(a);
rotate_right_extended!(b);
rotate_right_extended!(c);
rotate_right_extended!(d);
rotate_right_extended!(e);
rotate_right_extended!(h);
rotate_right_extended!(l);

/// Rotates a byte register left, moving the current value of the [`Flag::Carry`] flag into bit 0,
/// and the value of bit 7 into the carry register (`C <- [7 <- 0] <- C`).
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Zero**: Always unset
/// - **Subtract**: Always unset
/// - **Half-Carry**: Always unset
/// - **Carry**: The previous value of bit 7
pub fn rotate_left_carry(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let value = registers.get_byte(reg);
    let carry = value & 0x80 != 0;

    registers.set_byte(reg, (value << 1) | registers.get_flag(Flag::Carry) as u8);

    registers.clear_flags();
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Same as [`rotate_left_carry()`], except it also sets the [`Flag::Zero`] flag.
pub fn rotate_left_carry_extended(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let effect = rotate_left_carry(registers, reg);
    registers.set_flag(Flag::Zero, registers.get_byte(reg) == 0);

    effect
}

/// Implementation of [`rotate_left_carry()`] for A
pub fn rotate_left_carry_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_left_carry(registers, ByteRegister::A)
}

rotate_left_carry_extended!(a);
rotate_left_carry_extended!(b);
rotate_left_carry_extended!(c);
rotate_left_carry_extended!(d);
rotate_left_carry_extended!(e);
rotate_left_carry_extended!(h);
rotate_left_carry_extended!(l);

/// Rotates a byte register right, moving the current value of the [`Flag::Carry`] flag into bit 7,
/// and the value of bit 0 into the carry register (`C -> [7 -> 0] -> C`).
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Zero**: Always unset
/// - **Subtract**: Always unset
/// - **Half-Carry**: Always unset
/// - **Carry**: The previous value of bit 0
pub fn rotate_right_carry(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let value = registers.get_byte(reg);
    let carry = value & 1 != 0;

    registers.set_byte(
        reg,
        (value >> 1) | (registers.get_flag(Flag::Carry) as u8) << 7,
    );

    registers.clear_flags();
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Implementation of [`rotate_right_carry()`] for A
pub fn rotate_right_carry_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    rotate_right_carry(registers, ByteRegister::A)
}

/// Like [`rotate_right_carry()`], but the [`Flag::Zero`] flag is also set based on the result.
pub fn rotate_right_carry_extended(registers: &mut Registers, reg: ByteRegister) -> Effect {
    let effect = rotate_right_carry(registers, reg);
    registers.set_flag(Flag::Zero, registers.get_byte(reg) == 0);

    effect
}

rotate_right_carry_extended!(a);
rotate_right_carry_extended!(b);
rotate_right_carry_extended!(c);
rotate_right_carry_extended!(d);
rotate_right_carry_extended!(e);
rotate_right_carry_extended!(h);
rotate_right_carry_extended!(l);

/// Complements (bitwise negates) the the accumulator (A) register.
///
/// T-states: 4
/// M-cylces: 1
/// Width: 1
///
/// Flags:
/// - **Subtract**: Always set
/// - **Half Carry**: Always set
pub fn complement_a(registers: &mut Registers, _: &mut Memory) -> Effect {
    registers.a = !registers.a;

    registers.set_flag(Flag::Subtract, true);
    registers.set_flag(Flag::HalfCarry, true);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}
