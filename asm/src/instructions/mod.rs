use self::add::Add;
use self::decrement::Decrement;
use self::increment::Increment;
use self::jump::Jump;
use self::load::Load;
use self::rotate_left::RotateLeft;
use self::rotate_right::RotateRight;
use crate::operations::Operation;
use crate::parse::{Parse, ParseResult};
use crate::{parse_helper, read::Read, register_helper, sets};
use parse_display::Display;

pub mod add;
pub mod decrement;
pub mod increment;
pub mod jump;
pub mod load;
pub mod rotate_left;
pub mod rotate_right;

#[derive(Debug, Clone, Copy, Display)]
#[display("{0}")]
pub enum Instruction {
    #[display("NOP")]
    Nop,
    #[display("STOP")]
    Stop,
    Load(Load),
    Increment(Increment),
    Decrement(Decrement),
    RotateLeft(RotateLeft),
    RotateRight(RotateRight),
    Add(Add),
    Jump(Jump),
}

impl Instruction {
    pub const fn set() -> sets::Instructions {
        let mut builder = sets::Builder::default();
        builder.base(0x00, Instruction::Nop);
        builder.base(0x10, Instruction::Stop);

        register_helper!(
            &mut builder,
            Load,
            Increment,
            Decrement,
            RotateLeft,
            Add,
            RotateRight,
            Jump,
        );

        builder.build()
    }
}

impl Parse for Instruction {
    fn parse<R: Read>(&self, data: &R, offset: u16) -> ParseResult {
        parse_helper!(
            self,
            data[offset],
            Self::Nop => Operation::Nop,
            Self::Stop => Operation::Stop,
            Self::Load(inner),
            Self::Increment(inner),
            Self::Decrement(inner),
            Self::RotateLeft(inner),
            Self::RotateRight(inner),
            Self::Add(inner),
            Self::Jump(inner),
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
