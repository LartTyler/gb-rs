use super::Effect;
use crate::registers::{Flag, Registers};
use gb_rs_core::bytes::is_half_carry;
use gb_rs_cpu_macros::sub_r8_from_a;

fn sub_from_a(registers: &mut Registers, value: u8) -> Effect {
    let (new_a, overflow) = registers.a.overflowing_sub(value);
    registers.a = new_a;

    registers.set_flag(Flag::Zero, new_a == 0);
    registers.set_flag(Flag::Subtract, true);
    registers.set_flag(Flag::Carry, overflow);
    registers.set_flag(Flag::HalfCarry, is_half_carry(new_a, value));

    Effect {
        t_states: 4,
        width_bytes: 1,
    }
}

// ====== SUB r8 =======
sub_r8_from_a!(a);
sub_r8_from_a!(b);
sub_r8_from_a!(c);
sub_r8_from_a!(d);
sub_r8_from_a!(e);
sub_r8_from_a!(h);
sub_r8_from_a!(l);
