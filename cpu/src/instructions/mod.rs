use crate::Cpu;
use gb_rs_asm::{containers::Cycles, operations::OperationKind};
use gb_rs_memory::Memory;
use std::convert::TryInto;

mod add;
mod bit;
mod compare;
mod decrement;
mod increment;
mod jump;
mod load;
mod rotate_left;
mod rotate_right;
mod stack;
mod subroutine;
mod subtract;

pub struct Effect {
    pub cycles: u8,
}

/// Convenience conversion for [`Cycles`] into an [`Effect`].
///
/// This conversion will panic if the [`Cycles`] enum being converted is not [`Cycles::Fixed`].
impl From<Cycles> for Effect {
    fn from(value: Cycles) -> Self {
        Self {
            cycles: value
                .try_into()
                .expect("this instruction should never branch"),
        }
    }
}

pub trait Execute {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect;
}

#[macro_export(local_inner_macros)]
macro_rules! enum_pass_execute {
    ( $( $enum:ident :: $variant:ident $( ($inner:ident) )? $( => $retval:expr )? $(,)? ),* ) => {
        fn execute(
            self,
            cpu: &mut $crate::Cpu,
            memory: &mut gb_rs_memory::Memory,
            cycles: gb_rs_asm::containers::Cycles,
        ) -> $crate::instructions::Effect {
            match self {
                $( $enum::$variant $( ($inner) )? => parse_pass_arm_rhs!( cpu, memory, cycles => $( $inner )? $( $retval )? ) ),*
            }
        }
    };
}

#[macro_export]
macro_rules! parse_pass_arm_rhs {
    ( $cpu:ident, $memory:ident, $cycles:ident => $inner:ident ) => {
        $inner.execute($cpu, $memory, $cycles)
    };

    ( $cpu:ident, $memory:ident, $cycles:ident => $retval:expr ) => {
        $retval
    };
}

impl Execute for OperationKind {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        match self {
            Self::Nop => Effect { cycles: 1 },
            Self::Stop => do_stop(cpu, memory),
            Self::Halt => do_halt(cpu, memory),
            Self::DecimalAdjust => do_decimal_adjust(cpu, memory),
            Self::DisableInterrupts => {
                cpu.interrupts_enabled = false;
                Effect { cycles: 1 }
            }
            Self::EnableInterrupts => {
                cpu.interrupts_enabled = true;
                Effect { cycles: 1 }
            }
            Self::Load(inner) => inner.execute(cpu, memory, cycles),
            Self::Increment(inner) => inner.execute(cpu, memory, cycles),
            Self::Decrement(inner) => inner.execute(cpu, memory, cycles),
            Self::RotateLeft(inner) => inner.execute(cpu, memory, cycles),
            Self::RotateRight(inner) => inner.execute(cpu, memory, cycles),
            Self::Add(inner) => inner.execute(cpu, memory, cycles),
            Self::Subtract(inner) => inner.execute(cpu, memory, cycles),
            Self::Compare(inner) => inner.execute(cpu, memory, cycles),
            Self::Jump(inner) => inner.execute(cpu, memory, cycles),
            Self::Bit(inner) => inner.execute(cpu, memory, cycles),
            Self::Subroutine(inner) => inner.execute(cpu, memory, cycles),
            Self::Stack(inner) => inner.execute(cpu, memory, cycles),
        }
    }
}

fn do_stop(_cpu: &mut Cpu, _memory: &mut Memory) -> Effect {
    todo!()
}

fn do_halt(_cpu: &mut Cpu, _memory: &mut Memory) -> Effect {
    todo!()
}

fn do_decimal_adjust(_cpu: &mut Cpu, _memory: &mut Memory) -> Effect {
    todo!()
}
