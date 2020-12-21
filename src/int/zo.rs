pub trait ZeroOne: Copy + Eq {
	const ZERO: Self;
	fn is_zero(self) -> bool {
		self == Self::ZERO
	}
	const ONE: Self;
}

macro_rules! impl_zo {
	($($t:ty),*) => { $(
		impl ZeroOne for $t {
			const ZERO: Self = 0;
			const ONE: Self = 1;
		}
	)* };
}

impl_zo!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
