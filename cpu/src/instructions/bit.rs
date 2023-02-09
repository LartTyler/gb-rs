use super::{Effect, Execute};
use crate::Cpu;
use gb_rs_asm::{
    containers::{Cycles, Flag},
    instructions::bit::{
        BitwiseReset, BitwiseResetTarget, BitwiseSet, BitwiseSetTarget, BitwiseTest,
        BitwiseTestTarget, Complement, ComplementTarget, ShiftLeft, ShiftLeftTarget, ShiftRight,
        ShiftRightTarget, Swap, SwapTarget,
    },
    operations::bit::{
        Bit, BitwiseAnd, BitwiseAndTarget, BitwiseOr, BitwiseOrTarget, BitwiseXor, BitwiseXorTarget,
    },
};
use gb_rs_memory::Memory;

impl Execute for Bit {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        match self {
            Self::SetCarryFlag => {
                cpu.registers.flags.set(Flag::Carry);
                cpu.registers.flags.unset(Flag::Subtract);
                cpu.registers.flags.unset(Flag::HalfCarry);

                Effect { cycles: 1 }
            }
            Self::Complement(inner) => inner.execute(cpu, memory, cycles),
            _ => todo!(),
        }
    }
}

impl Execute for Complement {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use ComplementTarget::*;

        match self.target {
            Accumulator => {
                cpu.registers.a = !cpu.registers.a;

                cpu.registers.flags.set(Flag::Subtract);
                cpu.registers.flags.set(Flag::HalfCarry);
            }
            Carry => {
                cpu.registers.flags.toggle(Flag::Carry);

                cpu.registers.flags.unset(Flag::Subtract);
                cpu.registers.flags.unset(Flag::HalfCarry);
            }
        };

        cycles.into()
    }
}

impl Execute for BitwiseSet {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        match self.target {
            BitwiseSetTarget::Register(reg) => {
                let value = cpu.registers.get_byte(reg);
                cpu.registers.set_byte(reg, self.bit.with_set(value));
            }
            BitwiseSetTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);

                let value = memory.read_byte(addr);
                memory.write_byte(addr, self.bit.with_set(value));
            }
        };

        cycles.into()
    }
}

impl Execute for BitwiseReset {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        match self.target {
            BitwiseResetTarget::Register(reg) => {
                let value = cpu.registers.get_byte(reg);
                cpu.registers.set_byte(reg, self.bit.with_unset(value));
            }
            BitwiseResetTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);

                let value = memory.read_byte(addr);
                memory.write_byte(addr, self.bit.with_unset(value));
            }
        };

        cycles.into()
    }
}

impl Execute for BitwiseTest {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let is_set = match self.target {
            BitwiseTestTarget::Register(reg) => {
                let value = cpu.registers.get_byte(reg);

                self.bit.is_set(value)
            }
            BitwiseTestTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                let value = memory.read_byte(addr);

                self.bit.is_set(value)
            }
        };

        cpu.registers.flags.set_if(Flag::Zero, !is_set);
        cpu.registers.flags.unset(Flag::Subtract);
        cpu.registers.flags.set(Flag::HalfCarry);

        cycles.into()
    }
}

impl Execute for BitwiseAnd {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = match self.target {
            BitwiseAndTarget::Register(reg) => cpu.registers.get_byte(reg),
            BitwiseAndTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
            BitwiseAndTarget::Data(data) => *data,
        };

        cpu.registers.a &= value;

        cpu.registers.flags.reset();
        cpu.registers.flags.set(Flag::HalfCarry);
        cpu.registers.flags.set_if(Flag::Zero, cpu.registers.a == 0);

        cycles.into()
    }
}

impl Execute for BitwiseXor {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = match self.target {
            BitwiseXorTarget::Register(reg) => cpu.registers.get_byte(reg),
            BitwiseXorTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
            BitwiseXorTarget::Data(data) => *data,
        };

        cpu.registers.a ^= value;

        cpu.registers.flags.reset();
        cpu.registers.flags.set(Flag::HalfCarry);
        cpu.registers.flags.set_if(Flag::Zero, cpu.registers.a == 0);

        cycles.into()
    }
}

impl Execute for BitwiseOr {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = match self.target {
            BitwiseOrTarget::Register(reg) => cpu.registers.get_byte(reg),
            BitwiseOrTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
            BitwiseOrTarget::Data(data) => *data,
        };

        cpu.registers.a |= value;

        cpu.registers.flags.reset();
        cpu.registers.flags.set_if(Flag::Zero, cpu.registers.a == 0);

        cycles.into()
    }
}

impl Execute for ShiftLeft {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = match self.target {
            ShiftLeftTarget::Register(reg) => cpu.registers.get_byte(reg),
            ShiftLeftTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let carry = value & 0x80 > 0;
        let value = value << 1;

        cpu.registers.flags.reset();
        cpu.registers.flags.set_if(Flag::Carry, carry);
        cpu.registers.flags.set_if(Flag::Zero, value == 0);

        match self.target {
            ShiftLeftTarget::Register(reg) => cpu.registers.set_byte(reg, value),
            ShiftLeftTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.write_byte(addr, value);
            }
        };

        cycles.into()
    }
}

impl Execute for ShiftRight {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = match self.target {
            ShiftRightTarget::Register(reg) => cpu.registers.get_byte(reg),
            ShiftRightTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let carry = value & 1 > 0;
        let value = value >> 1;

        cpu.registers.flags.reset();
        cpu.registers.flags.set_if(Flag::Carry, carry);
        cpu.registers.flags.set_if(Flag::Zero, value == 0);

        match self.target {
            ShiftRightTarget::Register(reg) => cpu.registers.set_byte(reg, value),
            ShiftRightTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.write_byte(addr, value);
            }
        };

        cycles.into()
    }
}

impl Execute for Swap {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let value = match self.target {
            SwapTarget::Register(reg) => cpu.registers.get_byte(reg),
            SwapTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.read_byte(addr)
            }
        };

        let value = (value << 4) | (value >> 4);

        cpu.registers.flags.reset();
        cpu.registers.flags.set_if(Flag::Zero, value == 0);

        match self.target {
            SwapTarget::Register(reg) => cpu.registers.set_byte(reg, value),
            SwapTarget::PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer);
                memory.write_byte(addr, value);
            }
        };

        cycles.into()
    }
}
