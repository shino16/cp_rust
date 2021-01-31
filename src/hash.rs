use crate::slice::*;

pub struct Compress<T: Ord>(Vec<T>);

impl<T: Ord> Compress<T> {
	pub fn from(mut data: Vec<T>) -> Self {
		data.sort_unstable();
		data.dedup();
		Self(data)
	}
	pub fn compress(&self, v: &T) -> usize {
		debug_assert!(self.0.binary_search(v).is_ok());
		self.0.lower_bound(v)
	}
	pub fn restore(&self, i: usize) -> &T {
		&self.0[i]
	}
}
