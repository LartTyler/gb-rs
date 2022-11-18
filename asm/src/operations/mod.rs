use self::load::Load;

pub mod load;

pub enum Operation {
    Nop,
    Load(Load),
}
