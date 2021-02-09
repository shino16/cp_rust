use std::ops::*;

pub trait Bits:
    Sized
    + BitAnd<Output = Self> + BitAndAssign
    + BitOr<Output = Self> + BitOrAssign
    + BitXor<Output = Self> + BitXorAssign
    + Shl<u32, Output = Self> + ShlAssign<u32>
    + Shr<u32, Output = Self> + ShrAssign<u32>
    + Not<Output = Self>
{
    fn trailing_zeros(self) -> u32;
    fn lsb(self) -> Self;
    fn ilog2(self) -> u32;
    fn msb(self) -> Self;
}

macro_rules! impl_bit {
    ($($t:ty), *) => { $(
        impl Bits for $t {
            fn trailing_zeros(self) -> u32 {
                <$t>::trailing_zeros(self)
            }
            fn lsb(self) -> Self {
                self & self.wrapping_neg()
            }
            fn ilog2(self) -> u32 {
                std::mem::size_of::<$t>() as u32 * 8 - self.leading_zeros() - 1
            }
            fn msb(self) -> Self {
                (1 as $t) << self.ilog2()
            }
        }
    )* };
}

impl_bit!(i32, i64, i128, isize, u32, u64, u128, usize);
