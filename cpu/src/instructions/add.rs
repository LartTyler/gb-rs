use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, PairRegister, Registers};
use gb_rs_core::bytes::is_half_carry;
use gb_rs_cpu_macros::{add_r16_to_r16, add_r8_and_carry_to_a, add_r8_to_a};
use gb_rs_memory::Memory;

/// Adds two [`PairRegister`]s together, placing the result into `target`.
///
/// T-states: 8
/// M-cycles: 2
/// Width: 1
///
/// Flags:
/// - **Subtract**: Always unset
/// - **Half-Carry**: If there was a carry from bit 3 to bit 4
/// - **Carry**: If there was a carry out of bit 7
fn add_pair_to_pair(
    registers: &mut Registers,
    source: PairRegister,
    target: PairRegister,
) -> Effect {
    let lhs = registers.get_pair(target);
    let rhs = registers.get_pair(source);

    let (new_value, carry) = lhs.overflowing_add(rhs);
    registers.set_pair(target, new_value);

    registers.set_flag(Flag::Subtract, false);
    registers.set_flag(Flag::HalfCarry, is_half_carry(lhs, rhs));
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 8,
        width_bytes: 1,
    }
}

fn add_byte_to_a(registers: &mut Registers, rhs: u8) -> Effect {
    let lhs = registers.a;

    let (new_value, carry) = lhs.overflowing_add(rhs);
    registers.a = new_value;

    registers.set_flag(Flag::Subtract, false);
    registers.set_flag(Flag::HalfCarry, is_half_carry(lhs, rhs));
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

/// Adds a [`ByteRegister`] to the accumulator (`a` register), placing the result in `a`.
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - **Subtract**: Always unset
/// - **Half-Carry**: If there was a carry from bit 3 to bit 4
/// - **Carry**: If there was a carry out of bit 7
fn add_byte_register_to_a(registers: &mut Registers, source: ByteRegister) -> Effect {
    add_byte_to_a(registers, registers.get_byte(source))
}

pub fn add_hl_pointer_to_a(registers: &mut Registers, memory: &mut Memory) -> Effect {
    add_byte_to_a(
        registers,
        memory.read_byte(registers.get_pair(PairRegister::HL)),
    ) + Effect {
        t_states: 4,
        width_bytes: 0,
    }
}

fn add_byte_and_carry_to_a(registers: &mut Registers, rhs: u8) -> Effect {
    let carry = match registers.get_flag(Flag::Carry) {
        true => 1,
        _ => 0,
    };

    add_byte_to_a(registers, rhs.wrapping_add(carry))
}

// ===== ADD r16, r16 =====
add_r16_to_r16!(bc, hl);
add_r16_to_r16!(de, hl);
add_r16_to_r16!(hl, hl);
add_r16_to_r16!(sp, hl);

// ===== ADD A, r8 =====
add_r8_to_a!(a);
add_r8_to_a!(b);
add_r8_to_a!(c);
add_r8_to_a!(d);
add_r8_to_a!(e);
add_r8_to_a!(h);
add_r8_to_a!(l);

// ===== ADC A, r8 =====
add_r8_and_carry_to_a!(a);
add_r8_and_carry_to_a!(b);
add_r8_and_carry_to_a!(c);
add_r8_and_carry_to_a!(d);
add_r8_and_carry_to_a!(e);
add_r8_and_carry_to_a!(h);
add_r8_and_carry_to_a!(l);

pub fn add_hl_pointer_and_carry_to_a(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let carry = match registers.get_flag(Flag::Carry) {
        true => 1,
        _ => 0,
    };

    add_byte_and_carry_to_a(
        registers,
        memory
            .read_byte(registers.get_pair(PairRegister::HL))
            .wrapping_add(carry),
    );

    Effect {
        t_states: 8,
        width_bytes: 1,
    }
}
