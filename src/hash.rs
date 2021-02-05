use crate::slice::*;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Compress<T: Ord>(Vec<T>);

impl<T: Ord> Compress<T> {
	pub fn from(mut data: Vec<T>) -> Self {
		data.sort_unstable();
		data.dedup();
		Self(data)
	}
	pub fn compress(&self, v: &T) -> usize {
		debug_assert!(self.0.binary_search(v).is_ok());
		lower_bound(&self.0, v)
	}
	pub fn restore(&self, i: usize) -> &T {
		&self.0[i]
	}
	pub fn cache_al(&self) -> HashMap<T, usize>
	where
		T: Clone + Hash,
	{
		self.0.iter().cloned().zip(0..).collect()
	}
}
