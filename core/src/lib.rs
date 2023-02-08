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

impl<T> MathResult<T> {
    /// Merges two [`MathResult`]s using the following semantics:
    /// - The resulting `half_carry` is `true` if either `half_carry` was `true`.
    /// - The resulting `carry` is `true` if either `carry` was `true`.
    /// - The `result` is set to the value of the right-hand side's `result`.
    ///
    /// This function is intended to be used with ADC and SBC instructions, where the operation is
    /// implemented using a normal add / subtract, followed by the same operation on the value of
    /// the [`Flag::Carry`] flag.
    pub fn merge(self, other: Self) -> Self {
        Self {
            half_carry: self.half_carry || other.half_carry,
            carry: self.carry || other.carry,
            result: other.result,
        }
    }
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

pub trait Z80Sub<Rhs = Self> {
    type Output;

    fn sub_with_flags(self, rhs: Rhs) -> MathResult<Self::Output>;
}

impl Z80Sub for u8 {
    type Output = Self;

    fn sub_with_flags(self, rhs: Self) -> MathResult<Self::Output> {
        let half_carry = ((self & 0xF).wrapping_sub(rhs & 0xF) & 0x10) > 0;
        let (result, carry) = self.overflowing_sub(rhs);

        MathResult {
            half_carry,
            carry,
            result,
        }
    }
}

impl Z80Sub for u16 {
    type Output = Self;

    fn sub_with_flags(self, rhs: Self) -> MathResult<Self::Output> {
        let half_carry = ((self & 0xF).wrapping_sub(rhs & 0xF) & 0x10) > 0;
        let (result, carry) = self.overflowing_sub(rhs);

        MathResult {
            half_carry,
            carry,
            result,
        }
    }
}
