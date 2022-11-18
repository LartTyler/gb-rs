use self::add::Add;
use self::load::Load;
use crate::instructions::decrement::Decrement;
use crate::instructions::increment::Increment;
use crate::instructions::rotate_left::RotateLeft;
use parse_display::Display;

pub mod add;
pub mod load;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Operation {
    #[display("NOP")]
    Nop,
    Load(Load),
    Increment(Increment),
    Decrement(Decrement),
    RotateLeft(RotateLeft),
    Add(Add),
}
