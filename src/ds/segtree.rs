pub mod lazy;
pub use crate::alg::*;
use std::ops::Index;
use std::slice::SliceIndex;

#[derive(Clone)]
pub struct SegmentTree<A: Alg> {
	len: usize,
	data: Vec<A::Item>,
	alg: A,
}

impl<A: Monoid> SegmentTree<A> {
	pub fn new(len: usize, alg: A) -> Self {
		Self {
			len,
			data: vec![alg.unit(); len * 2],
			alg,
		}
	}
	pub fn new_from_slice(slice: &[A::Item], alg: A) -> Self {
		let len = slice.len();
		let mut data = slice.to_vec();
		data.extend_from_slice(slice);
		for i in (1..len).rev() {
			data[i] = alg.op(data[i << 1], data[i << 1 | 1]);
		}
		Self { len, data, alg }
	}
	pub fn len(&self) -> usize {
		self.len
	}
	fn build(&mut self, mut p: usize) {
		p >>= 1;
		while p != 0 {
			self.data[p] = self.alg.op(self.data[p << 1], self.data[p << 1 | 1]);
			p >>= 1;
		}
	}
	pub fn add(&mut self, pos: usize, v: A::Item) {
		let p = pos + self.len();
		self.data[p] = self.alg.op(self.data[p], v);
		self.build(p);
	}
	pub fn exec<F: FnOnce(&mut A::Item)>(&mut self, pos: usize, f: F) {
		let p = pos + self.len();
		f(&mut self.data[p]);
		self.build(p);
	}
	pub fn ask(&self, mut l: usize, mut r: usize) -> A::Item {
		let (mut resl, mut resr) = (self.alg.unit(), self.alg.unit());
		l += self.len();
		r += self.len();
		while l < r {
			if l & 1 != 0 {
				resl = self.alg.op(resl, self.data[l]);
				l += 1;
			}
			if r & 1 != 0 {
				resr = self.alg.op(self.data[r - 1], resr);
				r -= 1;
			}
			l >>= 1;
			r >>= 1;
		}
		self.alg.op(resl, resr)
	}
}

impl<A: Monoid, I: SliceIndex<[A::Item]>> Index<I> for SegmentTree<A> {
    type Output = I::Output;
    fn index(&self, idx: I) -> &Self::Output {
		&self.data[self.len()..][idx]
    }
}
