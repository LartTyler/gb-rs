use crate::registers::Registers;
use gb_rs_memory::Memory;
use std::ops::Add;

pub mod add;
pub mod bitwise;
pub mod decrement;
pub mod increment;
pub mod load;

/// Used to describe the effects of some instruction on the current device state.
pub struct Effect {
    /// The number of t-states the instruction took.
    ///
    /// Instruction time is also commonly referred to by its "m-cycles", which is just t-states
    /// divided by 4.
    pub t_states: u8,

    /// The instruction's width in bytes.
    ///
    /// This represents the number of bytes the instruction takes up in memory, and should be added
    /// to the current value of the program counter after the instruction executes in order to move
    /// the program to the next instruction.
    ///
    /// An instruction cannot have a width less than one byte.
    pub width_bytes: u8,
}

impl Add for Effect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            t_states: self.t_states + rhs.t_states,
            width_bytes: self.width_bytes + rhs.width_bytes,
        }
    }
}

pub type Instruction = fn(&mut Registers, &mut Memory) -> Effect;

pub fn get_instruction(opcode: u8) -> Option<Instruction> {
    Some(match opcode {
        0x00 => |_, _| Effect {
            t_states: 4,
            width_bytes: 1,
        },
        0x01 => load::load_immediate_into_bc, // LD BC, d16
        0x02 => load::load_a_into_bc_address, // LD (BC), A
        0x03 => increment::increment_bc,      // INC BC
        0x04 => increment::increment_b,       // INC B
        0x05 => decrement::decrement_b,       // DEC B
        0x06 => load::load_immediate_into_b,  // LD B, d8
        0x07 => bitwise::rotate_left_a,       // RLCA
        0x08 => load::load_sp_into_immediate_address, // LD (d16), SP
        0x09 => add::add_bc_to_hl,            // ADD HL, BC
        0x0A => load::load_bc_address_into_a, // LD A, (BC)
        0x0B => decrement::decrement_bc,      // DEC BC
        0x0C => increment::increment_c,       // INC C
        0x0D => decrement::decrement_c,       // DEC C
        0x0E => load::load_immediate_into_c,  // LD C, d8
        0x0F => bitwise::rotate_right_a,      // RRCA
        0x11 => load::load_immediate_into_de, // LD DE, d16
        0x12 => load::load_a_into_de_address, // LD (DE), A
        0x13 => increment::increment_de,      // INC DE
        0x14 => increment::increment_d,       // INC D
        0x15 => decrement::decrement_d,       // DEC D
        0x16 => load::load_immediate_into_d,  // LD D, d8
        0x19 => add::add_de_to_hl,            // ADD HL, DE
        0x1A => load::load_de_address_into_a, // LD A, (DE)
        0x1B => decrement::decrement_de,      // DEC DE
        0x1C => increment::increment_e,       // INC E
        0x1D => decrement::decrement_e,       // DEC E
        0x1E => load::load_immediate_into_e,  // LD E, d8
        0x21 => load::load_immediate_into_hl, // LD HL, d16
        0x23 => increment::increment_hl,      // INC HL
        0x24 => increment::increment_h,       // INC H
        0x25 => decrement::decrement_h,       // DEC H
        0x26 => load::load_immediate_into_h,  // LD H, d8
        0x29 => add::add_hl_to_hl,            // ADD HL, HL
        0x2B => decrement::decrement_hl,      // DEC HL
        0x2C => increment::increment_l,       // INC L
        0x2D => decrement::decrement_l,       // DEC L
        0x2E => load::load_immediate_into_l,  // LD L, d8
        0x31 => load::load_immediate_into_sp, // LD SP, d16
        0x33 => increment::increment_sp,      // INC SP
        0x39 => add::add_sp_to_hl,            // ADD HL, SP
        0x3B => decrement::decrement_sp,      // DEC SP
        0x3C => increment::increment_a,       // INC A
        0x3D => decrement::decrement_a,       // DEC A
        0x3E => load::load_immediate_into_a,  // LD A, d8
        0x46 => load::load_hl_address_into_b, // LD B, (HL)
        0x4E => load::load_hl_address_into_c, // LD C, (HL)
        0x56 => load::load_hl_address_into_d, // LD D, (HL)
        0x5E => load::load_hl_address_into_e, // LD E, (HL)
        0x66 => load::load_hl_address_into_h, // LD H, (HL)
        0x6E => load::load_hl_address_into_l, // LD L, (HL)
        0x7E => load::load_hl_address_into_a, // LD A, (HL)
        0xCB => |r, m| {
            // PREFIX CB
            let opcode = m.read_byte(r.program_counter + 1);
            let instruction = get_cb_instruction(opcode);

            if let Some(instruction) = instruction {
                Effect {
                    t_states: 4,
                    width_bytes: 1,
                } + instruction(r, m)
            } else {
                panic!(
                    "Unrecognized instruction opcode {:#x} at {:#x}",
                    opcode,
                    r.stack_pointer + 1
                );
            }
        },
        _ => return None,
    })
}

fn get_cb_instruction(opcode: u8) -> Option<Instruction> {
    Some(match opcode {
        0x00 => bitwise::rotate_left_b,           // RLC B
        0x01 => bitwise::rotate_left_c,           // RLC C
        0x02 => bitwise::rotate_left_d,           // RLC D
        0x03 => bitwise::rotate_left_e,           // RLC E
        0x04 => bitwise::rotate_left_h,           // RLC H
        0x05 => bitwise::rotate_left_l,           // RLC L
        0x07 => bitwise::rotate_left_a_extended,  // RLC A
        0x08 => bitwise::rotate_right_b,          // RRC B
        0x09 => bitwise::rotate_right_c,          // RRC C
        0x0A => bitwise::rotate_right_d,          // RRC D
        0x0B => bitwise::rotate_right_e,          // RRC E
        0x0C => bitwise::rotate_right_h,          // RRC H
        0x0D => bitwise::rotate_right_l,          // RRC L
        0x0F => bitwise::rotate_right_a_extended, // RRC A
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_instr {
        ( $opcode:expr ) => {
            assert_instr!(get_instruction, "", $opcode);
        };

        ( cb $opcode:expr ) => {
            assert_instr!(get_cb_instruction, "0xCB ", $opcode);
        };

        ( $callback:expr, $prefix:expr, $opcode:expr ) => {
            let instruction = $callback($opcode);
            assert!(
                instruction.is_some(),
                "Instruction {}{:#04X} not implemented",
                $prefix,
                $opcode
            );
        };
    }

    #[test]
    fn all_opcodes_implemented() {
        const EXCLUDED: [u8; 11] = [
            0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
        ];

        for opcode in 0..=0xFF {
            if EXCLUDED.contains(&opcode) {
                continue;
            }

            assert_instr!(opcode);
        }
    }

    #[test]
    fn all_cb_opcodes_implemented() {
        for opcode in 0..=0xFF {
            assert_instr!(cb opcode);
        }
    }
}
