use super::Load;
use crate::containers::{Pointer, Value};
use crate::enum_from_helper;
use crate::instructions::load as instr;
use crate::operations::Operation;
use parse_display::Display;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct DataPointerLoad {
    pub target: DataPointerLoadTarget,
    pub source: instr::DataPointerLoadSource,
}

impl DataPointerLoad {
    pub const fn create(
        target: DataPointerLoadTarget,
        source: instr::DataPointerLoadSource,
    ) -> Operation {
        Operation::Load(Load::DataPointer(Self { target, source }))
    }
}

impl Display for DataPointerLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let DataPointerLoadTarget::High(p) = self.target {
            write!(f, "LDH {p}, {}", self.source)
        } else {
            write!(f, "LD {}, {}", self.target, self.source)
        }
    }
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum DataPointerLoadTarget {
    Absolute(Pointer<Value<u16>>),
    High(Pointer<Value<u8>>),
}

enum_from_helper!(
    const Pointer<Value<u16>> => DataPointerLoadTarget::Absolute,
    const Pointer<Value<u8>> => DataPointerLoadTarget::High,
);
