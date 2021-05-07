pub trait ZeroOne: Copy + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(self) -> bool { self == Self::zero() }
}

macro_rules! impl_zo {
    ($($t:ty),*) => { $(
        impl ZeroOne for $t {
            fn zero() -> Self { 0 as $t }
            fn one() -> Self { 1 as $t }
        }
    )* };
}

impl_zo!(i32, i64, i128, isize, u32, u64, u128, usize, f32, f64);
