use crate::instructions::Effect;
use crate::registers::{ByteRegister, Flag, PairRegister, Registers};
use gb_rs_core::bytes::is_half_carry_add;
use gb_rs_cpu_macros::{add_r16_to_r16, add_r8_to_a};

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
    registers.set_flag(Flag::HalfCarry, is_half_carry_add(lhs, rhs));
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 8,
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
    let lhs = registers.a;
    let rhs = registers.get_byte(source);

    let (new_value, carry) = lhs.overflowing_add(rhs);
    registers.a = new_value;

    registers.set_flag(Flag::Subtract, false);
    registers.set_flag(Flag::HalfCarry, is_half_carry_add(lhs, rhs));
    registers.set_flag(Flag::Carry, carry);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
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
