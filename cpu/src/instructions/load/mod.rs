use crate::instructions::Effect;
use crate::registers::{ByteRegister, PairRegister, Registers};
use base::*;
use gb_rs_cpu_macros::{
    load_immediate_into_r16, load_immediate_into_r8, load_r16_pointer_into_r8,
    load_r8_into_r16_pointer, load_r8_into_r8_all,
};
use gb_rs_memory::Memory;

mod base;

// ===== LD r16, d16 =====
load_immediate_into_r16!(bc);
load_immediate_into_r16!(de);
load_immediate_into_r16!(hl);
load_immediate_into_r16!(sp);

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
load_immediate_into_r8!(a);
load_immediate_into_r8!(b);
load_immediate_into_r8!(c);
load_immediate_into_r8!(d);
load_immediate_into_r8!(e);
load_immediate_into_r8!(h);
load_immediate_into_r8!(l);

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
load_r16_pointer_into_r8!(bc, a);
load_r16_pointer_into_r8!(de, a);

load_r16_pointer_into_r8!(hl, a);
load_r16_pointer_into_r8!(hl, b);
load_r16_pointer_into_r8!(hl, c);
load_r16_pointer_into_r8!(hl, d);
load_r16_pointer_into_r8!(hl, e);
load_r16_pointer_into_r8!(hl, h);
load_r16_pointer_into_r8!(hl, l);

// ===== LD r8, r8 =====
load_r8_into_r8_all!(a);
load_r8_into_r8_all!(b);
load_r8_into_r8_all!(c);
load_r8_into_r8_all!(d);
load_r8_into_r8_all!(e);
load_r8_into_r8_all!(h);
load_r8_into_r8_all!(l);

// ===== LD (r16), r8 =====
load_r8_into_r16_pointer!(a, hl);
load_r8_into_r16_pointer!(b, hl);
load_r8_into_r16_pointer!(c, hl);
load_r8_into_r16_pointer!(d, hl);
load_r8_into_r16_pointer!(e, hl);
load_r8_into_r16_pointer!(h, hl);
load_r8_into_r16_pointer!(l, hl);

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
        registers.get_pair(PairRegister::HL).wrapping_sub(1),
    );

    effect
}

/// Implementation of [`load_pair_address_into_byte_register()`] for A, (HL+)
///
/// Also increments HL after the load.
pub fn load_hl_pointer_into_a_increment(registers: &mut Registers, memory: &mut Memory) -> Effect {
    let effect =
        load_pair_pointer_into_byte_register(registers, memory, PairRegister::HL, ByteRegister::A);

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
        load_pair_pointer_into_byte_register(registers, memory, PairRegister::HL, ByteRegister::A);

    registers.set_pair(
        PairRegister::HL,
        registers.get_pair(PairRegister::HL).wrapping_sub(1),
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
