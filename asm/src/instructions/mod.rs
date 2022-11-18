use self::add::Add;
use self::decrement::Decrement;
use self::increment::Increment;
use self::load::Load;
use self::rotate_left::RotateLeft;
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::{parse_helper, read::Read, register_helper, sets};
use parse_display::Display;

pub mod add;
pub mod decrement;
pub mod increment;
pub mod load;
pub mod rotate_left;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Instruction {
    #[display("NOP")]
    Nop,
    Load(Load),
    Increment(Increment),
    Decrement(Decrement),
    RotateLeft(RotateLeft),
    Add(Add),
}

impl Instruction {
    pub const fn set() -> sets::Instructions {
        let mut builder = sets::Builder::default();
        builder.base(0x00, Instruction::Nop);

        register_helper!(&mut builder, Load, Increment, Decrement, RotateLeft, Add);

        builder.build()
    }
}

impl Parse for Instruction {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Nop => Operation::Nop,
            Self::Load(inner),
            Self::Increment(inner),
            Self::Decrement(inner),
            Self::RotateLeft(inner),
            Self::Add(inner),
        )
    }
}

#[const_trait]
trait SetRegister {
    fn register(builder: &mut sets::Builder);
}

#[macro_export]
macro_rules! register_helper {
    ( $builder:ident , $( $item:ty $(,)? ),* ) => {
        $( <$item>::register($builder) );*
    };

    ( &mut $builder:ident , $( $item:ty $(,)? ),* ) => {
        $( <$item>::register(&mut $builder) );*
    }
}
