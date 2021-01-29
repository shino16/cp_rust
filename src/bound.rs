pub trait Bound: Ord {
	const MIN: Self;
	const MAX: Self;
}

macro_rules! impl_bound {
	($($t:ident),*) => { $(
		impl Bound for $t {
			const MIN: Self = std::$t::MIN;
			const MAX: Self = std::$t::MAX;
		}
	)* };
}

impl_bound!(i32, i64, i128, isize, u32, u64, u128, usize);
