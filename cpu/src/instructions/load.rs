use super::{Effect, Execute};
use crate::{enum_pass_execute, Cpu};
use gb_rs_asm::containers::{Cycles, Flag};
use gb_rs_asm::instructions::load::{Action, DataPointerLoadSource, RegisterPointerLoad};
use gb_rs_asm::operations::load::*;
use gb_rs_core::Z80Add;
use gb_rs_memory::Memory;

impl Execute for Load {
    enum_pass_execute!(
        Self::Pair(load),
        Self::PairPointer(load),
        Self::Register(load),
        Self::RegisterPointer(load),
        Self::DataPointer(load),
    );
}

impl Execute for PairLoad {
    fn execute(self, cpu: &mut Cpu, _memory: &mut Memory, cycles: Cycles) -> Effect {
        use PairLoadSource::*;

        let value = match self.source {
            Data(data) => *data,
            Pair(source) => cpu.registers.get_pair(source),
            SignedData(data) => {
                let initial_value = cpu.registers.stack_pointer;
                let output = initial_value.add_with_flags(data.as_twos_complement());

                cpu.registers.flags.reset();
                cpu.registers.flags.update_from_math_result(&output);

                output.result
            }
        };

        cpu.registers.set_pair(self.target, value);

        cycles.into()
    }
}

impl Execute for PairPointerLoad {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use PairPointerLoadSource::*;

        let value = match self.source {
            Data(n) => *n,
            Register(source) => cpu.registers.get_byte(source),
        };

        memory.write_byte(cpu.registers.get_pair(*self.target), value);

        match self.action {
            Action::Increment => {
                let value = cpu.registers.get_pair(*self.target).wrapping_add(1);
                cpu.registers.set_pair(*self.target, value);
            }
            Action::Decrement => {
                let value = cpu.registers.get_pair(*self.target).wrapping_sub(1);
                cpu.registers.set_pair(*self.target, value);
            }
            Action::None => (),
        };

        cycles.into()
    }
}

impl Execute for RegisterLoad {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use RegisterLoadSource::*;

        let value = match self.source {
            Data(n) => *n,
            PairPointer(pointer) => {
                let addr = cpu.registers.get_pair(*pointer.source);
                memory.read_byte(addr)
            }
            Register(source) => cpu.registers.get_byte(source),
            DataPointer(pointer) => memory.read_byte(pointer.into()),
            HighDataPointer(pointer) => {
                let value: u16 = pointer.into();
                let addr = 0xFF00 + value;

                memory.read_byte(addr)
            }
            RegisterPointer(pointer) => {
                let reg: u16 = cpu.registers.get_byte(*pointer).into();
                memory.read_byte(0xFF00 + reg)
            }
        };

        cpu.registers.set_byte(self.target, value);

        if let PairPointer(pointer) = self.source {
            match pointer.action {
                Action::Increment => {
                    let value = cpu.registers.get_pair(*pointer.source).wrapping_add(1);
                    cpu.registers.set_pair(*pointer.source, value);
                }
                Action::Decrement => {
                    let value = cpu.registers.get_pair(*pointer.source).wrapping_sub(1);
                    cpu.registers.set_pair(*pointer.source, value);
                }
                Action::None => (),
            };
        }

        cycles.into()
    }
}

impl Execute for RegisterPointerLoad {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        let reg: u16 = cpu.registers.c.into();
        let addr = 0xFF00 + reg;
        memory.write_byte(addr, cpu.registers.a);

        cycles.into()
    }
}

impl Execute for DataPointerLoad {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use DataPointerLoadTarget::*;

        let target_addr = match self.target {
            Absolute(pointer) => pointer.into(),
            High(pointer) => {
                let low_byte: u16 = pointer.into();
                0xFF00 + low_byte
            }
        };

        match self.source {
            DataPointerLoadSource::Register(source) => {
                memory.write_byte(target_addr, cpu.registers.get_byte(source));
            }
            DataPointerLoadSource::Pair(source) => {
                memory.write_word(target_addr, cpu.registers.get_pair(source));
            }
        };

        cycles.into()
    }
}
