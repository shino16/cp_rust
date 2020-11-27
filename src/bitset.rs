pub trait BitSet {
	fn bit_at(&self, i: usize) -> bool;
	fn set_bit_at(&mut self, i: usize, b: bool);
}

macro_rules! impl_bitset {
	($($type:ty),*) => { $(
		impl BitSet for $type {
			fn bit_at(&self, i: usize) -> bool {
				((*self >> i) & 1) != 0
			}
			fn set_bit_at(&mut self, i: usize, b: bool) {
				*self |= (b as $type) << i;
			}
		}
	)* };
}

impl_bitset!(i32, i64, i128, isize, u32, u64, u128, usize);

impl BitSet for [u32] {
	fn bit_at(&self, i: usize) -> bool {
		self[i / 32].bit_at(i % 32)
	}
	fn set_bit_at(&mut self, i: usize, b: bool) {
		self[i / 32].set_bit_at(i % 32, b);
	}
}
