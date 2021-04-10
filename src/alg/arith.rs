pub use super::*;
pub use crate::num::*;

#[derive(Default, Clone, Copy)]
pub struct Addition;

macro_rules! impl_alg {
    ($($t:ty),*) => { $(
        impl Monoid<$t> for Addition {
            fn unit(&self) -> $t { 0 }
            fn op(&self, x: $t, y: $t) -> $t { x.wrapping_add(y) }
        }
        impl Group<$t> for Addition {
            fn inv(&self, x: $t) -> $t { x.wrapping_neg() }
        }
    )* };
}

impl_alg!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
