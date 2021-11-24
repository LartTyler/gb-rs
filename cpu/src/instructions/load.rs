use crate::instructions::Effect;
use crate::registers::{ByteRegister, PairRegister, Registers};
use gb_rs_memory::Memory;

/// Writes an immediate word (2 byte value) from the current position in memory to the given
/// [`PairRegister`].
///
/// T-states: 12
/// M-cycles: 3
/// Width: 3
///
/// Flags:
/// - No flags changed
pub fn load_immediate_into_pair(
    registers: &mut Registers,
    memory: &Memory,
    pair: PairRegister,
) -> Effect {
    registers.set_pair(pair, memory.read_word(registers.program_counter + 1));

    Effect {
        t_states: 12,
        width_bytes: 3,
    }
}

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

/// Writes a byte into memory at the address pointed to by the given [`PairRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 1
///
/// Flags:
/// - No flags changed
pub fn load_byte_into_pair_address(
    registers: &mut Registers,
    memory: &mut Memory,
    pair: PairRegister,
    value: u8,
) -> Effect {
    memory.write_byte(registers.get_pair(pair), value);

    Effect {
        t_states: 8,
        width_bytes: 1,
    }
}

/// Implementation of [`load_byte_into_pair_address()`] for BC, A
pub fn load_a_into_bc_address(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_byte_into_pair_address(registers, memory, PairRegister::BC, registers.a)
}

/// Implementation of [`load_byte_into_pair_address()`] for DE, A
pub fn load_a_into_de_address(registers: &mut Registers, memory: &mut Memory) -> Effect {
    load_byte_into_pair_address(registers, memory, PairRegister::DE, registers.a)
}

/// Writes an immediate byte from the current position in memory to the given [`ByteRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 2
///
/// Flags:
/// - No flags changed
pub fn load_immediate_into_byte_register(
    registers: &mut Registers,
    memory: &Memory,
    reg: ByteRegister,
) -> Effect {
    registers.set_byte(reg, memory.read_byte(registers.program_counter + 1));

    Effect {
        t_states: 8,
        width_bytes: 2,
    }
}

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
