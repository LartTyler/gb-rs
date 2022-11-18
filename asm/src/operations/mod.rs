use self::load::Load;
use parse_display::Display;

pub mod load;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Operation {
    #[display("NOP")]
    Nop,
    Load(Load),
}
