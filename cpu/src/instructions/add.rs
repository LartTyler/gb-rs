use crate::instructions::Effect;
use crate::registers::{Flag, PairRegister, Registers};
use gb_rs_core::bytes::is_half_carry_add;
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
pub fn add_pair_to_pair(
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

/// Implementation of [`add_pair_to_pair()`] for HL, BC
pub fn add_bc_to_hl(registers: &mut Registers, _: &mut Memory) -> Effect {
    add_pair_to_pair(registers, PairRegister::BC, PairRegister::HL)
}

/// Implementation of [`add_pair_to_pair()`] for HL, DE
pub fn add_de_to_hl(registers: &mut Registers, _: &mut Memory) -> Effect {
    add_pair_to_pair(registers, PairRegister::DE, PairRegister::HL)
}

/// Implementation of [`add_pair_to_pair()`] for HL, HL
pub fn add_hl_to_hl(registers: &mut Registers, _: &mut Memory) -> Effect {
    add_pair_to_pair(registers, PairRegister::HL, PairRegister::HL)
}

/// Implementation of [`add_pair_to_pair()`] for HL, SP
pub fn add_sp_to_hl(registers: &mut Registers, _: &mut Memory) -> Effect {
    add_pair_to_pair(registers, PairRegister::SP, PairRegister::HL)
}
