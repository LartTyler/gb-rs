use super::SetRegister;
use crate::operations::Operation;
use crate::{parse, parse_helper, read, sets};

pub use pair::*;

mod pair;

#[derive(Debug, Clone, Copy)]
pub enum Load {
    Pair(PairLoad),
}

impl parse::Parse for Load {
    fn parse<R: read::Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        parse_helper!(self, data[offset], Self::Pair(inner))
    }
}

impl const SetRegister for Load {
    fn register(builder: &mut sets::Builder) {
        PairLoad::register(builder);
    }
}
