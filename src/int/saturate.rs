use std::ops::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Saturate<T>(pub T);

macro_rules! impl_ops {
    ($($t:ty),*) => { $(
        impl Add for Saturate<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }
        }
        impl AddAssign for Saturate<$t> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
    )* };
}

impl_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
