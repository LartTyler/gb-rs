pub mod bytes;
pub mod config;

#[derive(Copy, Clone)]
pub enum DeviceMode {
    Color,
    Classic,
}

pub struct MathResult<T> {
    pub half_carry: bool,
    pub carry: bool,
    pub result: T,
}

pub trait Z80Add<Rhs = Self> {
    type Output;

    fn add_with_flags(self, rhs: Rhs) -> MathResult<Self::Output>;
}

impl Z80Add for u8 {
    type Output = Self;

    fn add_with_flags(self, rhs: Self) -> MathResult<Self::Output> {
        let half_carry = (((self & 0xF) + (rhs & 0xF)) & 0x10) > 0;
        let (result, carry) = self.overflowing_add(rhs);

        MathResult {
            half_carry,
            carry,
            result,
        }
    }
}

impl Z80Add for u16 {
    type Output = Self;

    fn add_with_flags(self, rhs: Self) -> MathResult<Self::Output> {
        let half_carry = (((self & 0xF) + (rhs & 0xF)) & 0x10) > 0;
        let (result, carry) = self.overflowing_add(rhs);

        MathResult {
            half_carry,
            carry,
            result,
        }
    }
}

impl Z80Add<u8> for u16 {
    type Output = Self;

    fn add_with_flags(self, rhs: u8) -> MathResult<Self::Output> {
        // we can just cast rhs to a u16 and redirect to the u16 + u16 implementation
        self.add_with_flags(rhs as u16)
    }
}

impl Z80Add<i16> for u16 {
    type Output = Self;

    fn add_with_flags(self, rhs: i16) -> MathResult<Self::Output> {
        self.add_with_flags(rhs as u16)
    }
}
