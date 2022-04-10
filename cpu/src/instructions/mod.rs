use crate::registers::{Flag, Registers};
use gb_rs_memory::Memory;
use std::ops::Add;

pub mod add;
pub mod alu;
pub mod bitwise;
pub mod decrement;
pub mod increment;
pub mod jump;
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
        0x0A => load::load_bc_pointer_into_a, // LD A, (BC)
        0x0B => decrement::decrement_bc,      // DEC BC
        0x0C => increment::increment_c,       // INC C
        0x0D => decrement::decrement_c,       // DEC C
        0x0E => load::load_immediate_into_c,  // LD C, d8
        0x0F => bitwise::rotate_right_a,      // RRCA
        0x10 => |r: &mut Registers, _| {
            // STOP 0
            r.stop_flag = true;

            Effect {
                t_states: 4,
                width_bytes: 2,
            }
        },
        0x11 => load::load_immediate_into_de, // LD DE, d16
        0x12 => load::load_a_into_de_address, // LD (DE), A
        0x13 => increment::increment_de,      // INC DE
        0x14 => increment::increment_d,       // INC D
        0x15 => decrement::decrement_d,       // DEC D
        0x16 => load::load_immediate_into_d,  // LD D, d8
        0x17 => bitwise::rotate_left_carry_a, // RLA
        0x18 => jump::jump_relative_immediate_signed, // JR e8
        0x19 => add::add_de_to_hl,            // ADD HL, DE
        0x1A => load::load_de_pointer_into_a, // LD A, (DE)
        0x1B => decrement::decrement_de,      // DEC DE
        0x1C => increment::increment_e,       // INC E
        0x1D => decrement::decrement_e,       // DEC E
        0x1E => load::load_immediate_into_e,  // LD E, d8
        0x1F => bitwise::rotate_right_carry_a, // RRA
        0x20 => jump::jump_relative_zero_unset, // JR NZ, e8
        0x21 => load::load_immediate_into_hl, // LD HL, d16
        0x22 => load::load_a_into_hl_pointer_increment, // LD (HL+), A
        0x23 => increment::increment_hl,      // INC HL
        0x24 => increment::increment_h,       // INC H
        0x25 => decrement::decrement_h,       // DEC H
        0x26 => load::load_immediate_into_h,  // LD H, d8
        0x27 => alu::decimal_adjust_accum,    // DAA
        0x28 => jump::jump_relative_zero_set, // JR Z, e8
        0x29 => add::add_hl_to_hl,            // ADD HL, HL
        0x2A => load::load_hl_pointer_into_a_increment, // LD A, (HL+)
        0x2B => decrement::decrement_hl,      // DEC HL
        0x2C => increment::increment_l,       // INC L
        0x2D => decrement::decrement_l,       // DEC L
        0x2E => load::load_immediate_into_l,  // LD L, d8
        0x2F => bitwise::complement_a,        // CPL
        0x30 => jump::jump_relative_carry_unset, // JR NC, e8
        0x31 => load::load_immediate_into_sp, // LD SP, d16
        0x32 => load::load_a_into_hl_pointer_decrement, // LD (HL-), A
        0x33 => increment::increment_sp,      // INC SP
        0x34 => increment::increment_hl_pointer, // INC (HL)
        0x35 => decrement::decrement_hl_pointer, // DEC (HL)
        0x36 => load::load_immediate_into_hl_address, // LD (HL), d8
        0x37 => |r, _| {
            // SCF
            r.set_flag(Flag::Subtract, false);
            r.set_flag(Flag::HalfCarry, false);
            r.set_flag(Flag::Carry, true);

            Effect {
                t_states: 4,
                width_bytes: 1,
            }
        },
        0x38 => jump::jump_relative_carry_set, // JR C, e8
        0x39 => add::add_sp_to_hl,             // ADD HL, SP
        0x3A => load::load_hl_pointer_into_a_decrement, // LD A, (HL-)
        0x3B => decrement::decrement_sp,       // DEC SP
        0x3C => increment::increment_a,        // INC A
        0x3D => decrement::decrement_a,        // DEC A
        0x3E => load::load_immediate_into_a,   // LD A, d8
        0x3F => |r, _| {
            // CCF
            r.set_flag(Flag::Subtract, false);
            r.set_flag(Flag::HalfCarry, false);
            r.set_flag(Flag::Carry, !r.get_flag(Flag::Carry));

            Effect {
                t_states: 4,
                width_bytes: 1,
            }
        },
        0x40 => load::load_b_into_b,          // LD B, B
        0x41 => load::load_c_into_b,          // LD B, C
        0x42 => load::load_d_into_b,          // LD B, D
        0x43 => load::load_e_into_b,          // LD B, E
        0x44 => load::load_h_into_b,          // LD B, H
        0x45 => load::load_l_into_b,          // LD B, L
        0x46 => load::load_hl_pointer_into_b, // LD B, (HL)
        0x47 => load::load_a_into_b,          // LD B, A
        0x48 => load::load_b_into_c,          // LD C, B
        0x49 => load::load_c_into_c,          // LD C, C
        0x4A => load::load_d_into_c,          // LD C, D
        0x4B => load::load_e_into_c,          // LD C, E
        0x4C => load::load_h_into_c,          // LD C, H
        0x4D => load::load_l_into_c,          // LD C, L
        0x4E => load::load_hl_pointer_into_c, // LD C, (HL)
        0x4F => load::load_a_into_c,          // LD C, A
        0x50 => load::load_b_into_d,          // LD D, B
        0x51 => load::load_c_into_d,          // LD D, C
        0x52 => load::load_d_into_d,          // LD D, D
        0x53 => load::load_e_into_d,          // LD D, E
        0x54 => load::load_h_into_d,          // LD D, H
        0x55 => load::load_l_into_d,          // LD D, L
        0x56 => load::load_hl_pointer_into_d, // LD D, (HL)
        0x57 => load::load_a_into_d,          // LD D, A
        0x58 => load::load_b_into_e,          // LD E, B
        0x59 => load::load_c_into_e,          // LD E, C
        0x5A => load::load_d_into_e,          // LD E, D
        0x5B => load::load_e_into_e,          // LD E, E
        0x5C => load::load_h_into_e,          // LD E, H
        0x5D => load::load_l_into_e,          // LD E, L
        0x5E => load::load_hl_pointer_into_e, // LD E, (HL)
        0x5F => load::load_a_into_e,          // LD E, A
        0x60 => load::load_b_into_h,          // LD H, B
        0x61 => load::load_c_into_h,          // LD H, C
        0x62 => load::load_d_into_h,          // LD H, D
        0x63 => load::load_e_into_h,          // LD H, E
        0x64 => load::load_h_into_h,          // LD H, H
        0x65 => load::load_l_into_h,          // LD H, L
        0x66 => load::load_hl_pointer_into_h, // LD H, (HL)
        0x67 => load::load_a_into_h,          // LD H, A
        0x68 => load::load_b_into_l,          // LD L, B
        0x69 => load::load_c_into_l,          // LD L, C
        0x6A => load::load_d_into_l,          // LD D, L
        0x6B => load::load_e_into_l,          // LD L, E
        0x6C => load::load_h_into_l,          // LD L, H
        0x6D => load::load_l_into_l,          // LD L, L
        0x6E => load::load_hl_pointer_into_l, // LD L, (HL)
        0x6F => load::load_a_into_l,          // LD L, A
        0x7E => load::load_hl_pointer_into_a, // LD A, (HL)
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
        0x00 => bitwise::rotate_left_b_extended,        // RLC B
        0x01 => bitwise::rotate_left_c_extended,        // RLC C
        0x02 => bitwise::rotate_left_d_extended,        // RLC D
        0x03 => bitwise::rotate_left_e_extended,        // RLC E
        0x04 => bitwise::rotate_left_h_extended,        // RLC H
        0x05 => bitwise::rotate_left_l_extended,        // RLC L
        0x07 => bitwise::rotate_left_a_extended,        // RLC A
        0x08 => bitwise::rotate_right_b,                // RRC B
        0x09 => bitwise::rotate_right_c,                // RRC C
        0x0A => bitwise::rotate_right_d,                // RRC D
        0x0B => bitwise::rotate_right_e,                // RRC E
        0x0C => bitwise::rotate_right_h,                // RRC H
        0x0D => bitwise::rotate_right_l,                // RRC L
        0x0F => bitwise::rotate_right_a_extended,       // RRC A
        0x10 => bitwise::rotate_left_carry_b,           // RL B
        0x11 => bitwise::rotate_left_carry_c,           // RL C
        0x12 => bitwise::rotate_left_carry_d,           // RL D
        0x13 => bitwise::rotate_left_carry_e,           // RL E
        0x14 => bitwise::rotate_left_carry_h,           // RL H
        0x15 => bitwise::rotate_left_carry_l,           // RL L
        0x17 => bitwise::rotate_left_carry_a_extended,  // RL A
        0x18 => bitwise::rotate_right_carry_b,          // RR B
        0x19 => bitwise::rotate_right_carry_c,          // RR C
        0x1A => bitwise::rotate_right_carry_d,          // RR D
        0x1B => bitwise::rotate_right_carry_e,          // RR E
        0x1C => bitwise::rotate_right_carry_h,          // RR H
        0x1D => bitwise::rotate_right_carry_l,          // RR L
        0x1F => bitwise::rotate_right_carry_a_extended, // RR A
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
