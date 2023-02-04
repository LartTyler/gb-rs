use crate::Cpu;
use gb_rs_asm::{containers::Cycles, operations::OperationKind};
use gb_rs_memory::Memory;
use std::convert::TryInto;

mod load;

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

impl Execute for OperationKind {
    fn execute(self, cpu: &mut Cpu, memory: &mut Memory, cycles: Cycles) -> Effect {
        use OperationKind::*;

        match self {
            Nop => Effect { cycles: 1 },
            Load(load) => load.execute(cpu, memory, cycles),
            _ => todo!(),
        }
    }
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
