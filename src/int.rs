use crate::bit::*;
use std::ops::*;

pub trait ZeroOne: Copy + Eq {
    const ZERO: Self;
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    const ONE: Self;
}

pub trait Int:
    ZeroOne
    + Ord
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ShlAssign<u32>
    + ShrAssign<u32>
    + Bit
{
}

pub trait IInt: Int + Neg<Output = Self> {}
pub trait UInt: Int {}

macro_rules! impl_int {
    ($($ts:ty),*) => { $(
        impl ZeroOne for $ts {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
        impl Int for $ts {}
    )* };
    ($tr:ty | $($ts:ty),*) => {
        impl_int!($($ts),*);
        $( impl $tr for $ts {} )*
    }
}

impl_int!(IInt | i32, i64, i128, isize);
impl_int!(UInt | u32, u64, u128, usize);
