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
pub(super) fn load_immediate_into_pair(
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

/// Writes a byte into memory at the address pointed to by the given [`PairRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 1
///
/// Flags:
/// - No flags changed
pub(super) fn load_byte_into_pair_address(
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

/// Writes an immediate byte from the current position in memory to the given [`ByteRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 2
///
/// Flags:
/// - No flags changed
pub(super) fn load_immediate_into_byte_register(
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

/// Loads the byte pointed to by a [`PairRegister`] into a [`ByteRegister`].
///
/// T-states: 8
/// M-cycles: 2
/// Width: 1
///
/// Flags:
/// - No flags changed
pub(super) fn load_pair_address_into_byte_register(
    registers: &mut Registers,
    memory: &mut Memory,
    target: ByteRegister,
    source: PairRegister,
) -> Effect {
    let address = registers.get_pair(source);
    registers.set_byte(target, memory.read_byte(address));

    Effect {
        t_states: 8,
        width_bytes: 1,
    }
}

/// Writes the given byte value to a [`ByteRegister`].
///
/// T-states: 4
/// M-cycles: 1
/// Width: 1
///
/// Flags:
/// - No flags changed
pub(super) fn load_byte_into_byte_register(
    registers: &mut Registers,
    target: ByteRegister,
    value: u8,
) -> Effect {
    registers.set_byte(target, value);

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}
