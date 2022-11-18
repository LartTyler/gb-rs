use parse_display::Display;

pub use complement::*;

mod complement;

#[derive(Debug, Clone, Display)]
#[display("{0}")]
pub enum Bitwise {
    Complement(BitwiseComplement),
}
