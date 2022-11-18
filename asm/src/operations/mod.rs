use self::load::Load;
use crate::instructions::decrement::Decrement;
use crate::instructions::increment::Increment;
use parse_display::Display;

pub mod load;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Operation {
    #[display("NOP")]
    Nop,
    Load(Load),
    Increment(Increment),
    Decrement(Decrement),
}
