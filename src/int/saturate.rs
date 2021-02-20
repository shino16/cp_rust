use std::ops::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Saturate<T>(pub T);

impl<T> From<T> for Saturate<T> { fn from(x: T) -> Self { Self(x) } }

macro_rules! impl_ops {
    ($($t:ty),*) => { $(
        impl<T: Into<Saturate<$t>>> Add<T> for Saturate<$t> {
            type Output = Self;
            fn add(self, rhs: T) -> Self { Self(self.0.saturating_add(rhs.into().0)) }
        }
        impl<T: Into<Saturate<$t>>> AddAssign<T> for Saturate<$t> {
            fn add_assign(&mut self, rhs: T) { *self = *self + rhs; }
        }
    )* };
}

impl_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
