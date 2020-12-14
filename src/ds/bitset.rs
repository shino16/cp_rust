pub trait BitSet {
	fn get_bit(&self, i: usize) -> bool;
	fn set_bit(&mut self, i: usize, b: bool);
	fn modify_bit(&mut self, i: usize, b: bool) -> bool {
		let ret = self.get_bit(i);
		self.set_bit(i, b);
		ret
	}
}

macro_rules! impl_bitset {
	($($type:ty),*) => { $(
		impl BitSet for $type {
			fn get_bit(&self, i: usize) -> bool {
				((*self >> i) & 1) != 0
			}
			fn set_bit(&mut self, i: usize, b: bool) {
				*self |= (b as $type) << i;
			}
		}
	)* };
}

impl_bitset!(i32, i64, i128, isize, u32, u64, u128, usize);

impl BitSet for [u32] {
	fn get_bit(&self, i: usize) -> bool {
		self[i / 32].get_bit(i % 32)
	}
	fn set_bit(&mut self, i: usize, b: bool) {
		self[i / 32].set_bit(i % 32, b);
	}
}

pub fn new_bitset(n: usize) -> Vec<u32> {
	vec![0; n / 32]
}
