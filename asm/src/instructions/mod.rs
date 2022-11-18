use self::load::Load;
use crate::{operations::Operation, parse, parse_helper, read, sets};

pub mod load;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Nop,
    Load(Load),
}

impl Instruction {
    pub const fn set() -> sets::Instructions {
        let mut builder = sets::Builder::default();
        builder.base(0x00, Instruction::Nop);

        Load::register(&mut builder);

        builder.build()
    }
}

impl parse::Parse for Instruction {
    fn parse<R: read::Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        parse_helper!(
            self,
            data[offset],
            Self::Nop => Operation::Nop,
            Self::Load(inner)
        )
    }
}

#[const_trait]
trait SetRegister {
    fn register(builder: &mut sets::Builder);
}
