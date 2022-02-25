use crate::instructions::Effect;
use crate::registers::{ByteRegister, PairRegister, Registers};
use base::*;
use gb_rs_memory::Memory;

mod base;

// ===== LD r16, d16 =====

/// Implementation of [`load_immediate_into_pair()`] for BC
pub fn load_immediate_into_bc(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_pair(registers, memory, PairRegister::BC)
}

/// Implementation of [`load_immediate_into_pair()`] for DE
pub fn load_immediate_into_de(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_pair(registers, memory, PairRegister::DE)
}

/// Implementation of [`load_immediate_into_pair()`] for HL
pub fn load_immediate_into_hl(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_pair(registers, memory, PairRegister::HL)
}

/// Implementation of [`load_immediate_into_pair()`] for SP
pub fn load_immediate_into_sp(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_pair(registers, memory, PairRegister::SP)
}

// ===== LD r16, r8 =====

/// Implementation of [`load_byte_into_pair_address()`] for BC, A
pub fn load_a_into_bc_address(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_byte_into_pair_address(registers, memory, PairRegister::BC, registers.a)
}

/// Implementation of [`load_byte_into_pair_address()`] for DE, A
pub fn load_a_into_de_address(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_byte_into_pair_address(registers, memory, PairRegister::DE, registers.a)
}

// ===== LD r8, d8 =====

/// Implementation of [`load_immediate_into_byte_register()`] for A
pub fn load_immediate_into_a(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::A)
}

/// Implementation of [`load_immediate_into_byte_register()`] for B
pub fn load_immediate_into_b(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::B)
}

/// Implementation of [`load_immediate_into_byte_register()`] for C
pub fn load_immediate_into_c(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::C)
}

/// Implementation of [`load_immediate_into_byte_register()`] for D
pub fn load_immediate_into_d(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::D)
}

/// Implementation of [`load_immediate_into_byte_register()`] for E
pub fn load_immediate_into_e(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::E)
}

/// Implementation of [`load_immediate_into_byte_register()`] for H
pub fn load_immediate_into_h(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::H)
}

/// Implementation of [`load_immediate_into_byte_register()`] for L
pub fn load_immediate_into_l(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_immediate_into_byte_register(registers, memory, ByteRegister::L)
}

/// Loads the value of the stack pointer into the address pointed to by an immediate word (16-bit)
/// value.
///
/// More accurately, loads the low byte of the stack pointer into the address pointed to by an
/// immediate byte, and the high byte of the stack pointer into the address pointed to by an
/// immediate + 1 byte.
///
/// T-states: 20
/// M-cycles: 5
/// Width: 3
///
/// Flags:
/// - No flags changed
pub fn load_sp_into_immediate_address(registers: &mut Registers, memory: &mut Memory) -> Effect {
    memory.write_word(
        memory.read_word(registers.program_counter + 1),
        registers.stack_pointer,
    );

    Effect {
        t_states: 20,
        width_bytes: 3,
    }
}

// ===== LD r8, (r16) =====

/// Implementation of [`load_pair_address_into_byte_register()`] for A, BC
pub fn load_bc_address_into_a(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::A, PairRegister::BC)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for A, DE
pub fn load_de_address_into_a(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::A, PairRegister::DE)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for A, HL
pub fn load_hl_address_into_a(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::A, PairRegister::HL)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for B, HL
pub fn load_hl_address_into_b(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::B, PairRegister::HL)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for C, HL
pub fn load_hl_address_into_c(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::C, PairRegister::HL)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for D, HL
pub fn load_hl_address_into_d(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::D, PairRegister::HL)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for E, HL
pub fn load_hl_address_into_e(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::E, PairRegister::HL)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for H, HL
pub fn load_hl_address_into_h(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::H, PairRegister::HL)
}

/// Implementation of [`load_pair_address_into_byte_register()`] for L, HL
pub fn load_hl_address_into_l(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_pair_address_into_byte_register(registers, memory, ByteRegister::L, PairRegister::HL)
}

// ===== LD r8, r8 =====

/// Implementation of [`load_byte_into_byte_register()`] for B, A
pub fn load_a_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::A),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for B, B
pub fn load_b_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::B),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for B, C
pub fn load_c_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::C),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for B, D
pub fn load_d_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::D),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for B, E
pub fn load_e_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::E),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for B, H
pub fn load_h_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::H),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for B, L
pub fn load_l_into_b(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::B,
        registers.get_byte(ByteRegister::L),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, A
pub fn load_a_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::A),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, B
pub fn load_b_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::B),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, C
pub fn load_c_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::C),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, D
pub fn load_d_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::D),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, E
pub fn load_e_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::E),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, H
pub fn load_h_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::H),
    )
}

/// Implementation of [`load_byte_into_byte_register()`] for C, L
pub fn load_l_into_c(registers: &mut Registers, _: &mut Memory) -> Effect {
    load_byte_into_byte_register(
        registers,
        ByteRegister::C,
        registers.get_byte(ByteRegister::L),
    )
}

// ===== Misc. Loads (e.g. LD then inc / dec) =====

/// Implementation of [`load_byte_into_pair_address()`] for (HL+), A
///
/// Also increments HL after the load.
pub fn load_a_into_hl_pointer_increment(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let effect = load_byte_into_pair_address(registers, memory, PairRegister::HL, registers.a);
    registers.set_pair(
        PairRegister::HL,
        registers.get_pair(PairRegister::HL).wrapping_add(1),
    );

    effect
}

/// Implementation of [`load_byte_into_pair_address()`] for (HL-), A
///
/// Also decrements HL after the load.
pub fn load_a_into_hl_pointer_decrement(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let effect = load_byte_into_pair_address(registers, memory, PairRegister::HL, registers.a);
    registers.set_pair(
        PairRegister::HL,
        registers.get_pair(PairRegister::HL).wrapping_add(1),
    );

    effect
}

/// Implementation of [`load_pair_address_into_byte_register()`] for A, (HL+)
///
/// Also increments HL after the load.
pub fn load_hl_pointer_into_a_increment(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let effect =
        load_pair_address_into_byte_register(registers, memory, ByteRegister::A, PairRegister::HL);

    registers.set_pair(
        PairRegister::HL,
        registers.get_pair(PairRegister::HL).wrapping_add(1),
    );

    effect
}

/// Implementation of [`load_pair_address_into_byte_register()`] for A, (HL-)
///
/// Also decrements HL after the load.
pub fn load_hl_pointer_into_a_decrement(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let effect =
        load_pair_address_into_byte_register(registers, memory, ByteRegister::A, PairRegister::HL);

    registers.set_pair(
        PairRegister::HL,
        registers.get_pair(PairRegister::HL).wrapping_add(1),
    );

    effect
}

/// Implementation of [`load_immediate_into_pair_address()`] for (HL), d8
pub fn load_immediate_into_hl_address(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_byte_into_pair_address(
        registers,
        memory,
        PairRegister::HL,
        memory.read_byte(registers.stack_pointer + 1),
    );

    Effect {
        t_states: 12,
        width_bytes: 2,
    }
}
