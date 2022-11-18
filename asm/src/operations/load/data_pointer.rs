use super::Load;
use crate::containers::{Pointer, Value};
use crate::instructions::load as instr;
use crate::operations::Operation;
use parse_display::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("LD {target}, {source}")]
pub struct DataPointerLoad {
    pub target: Pointer<Value<u16>>,
    pub source: instr::DataPointerLoadSource,
}

impl DataPointerLoad {
    pub const fn create(
        target: Pointer<Value<u16>>,
        source: instr::DataPointerLoadSource,
    ) -> Operation {
        Operation::Load(Load::DataPointer(Self { target, source }))
    }
}
