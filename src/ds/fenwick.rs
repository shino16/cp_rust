pub use crate::alg::arith::*;
use crate::bit::*;

#[derive(Clone)]
pub struct FenwickTree<A: Alg> {
	data: Vec<A::Item>,
	alg: A,
}

// A: Commutative
impl<A: Monoid> FenwickTree<A> {
	pub fn new(mut data: Vec<A::Item>, alg: A) -> Self {
		let len = data.len();
		data.insert(0, alg.unit());
		for i in 1..=len {
			if i + i.lsb() <= len {
				data[i + i.lsb()] = alg.op(data[i + i.lsb()].clone(), data[i].clone());
			}
		}
		Self { data, alg }
	}
	pub fn len(&self) -> usize {
		self.data.len() - 1
	}
	pub fn add(&mut self, pos: usize, v: A::Item) {
		let mut pos = pos + 1;
		while pos < self.data.len() {
			self.data[pos] = self.alg.op(self.data[pos].clone(), v.clone());
			pos += pos.lsb();
		}
	}
	pub fn push(&mut self, v: A::Item) {
		self.data.push(self.alg.unit());
		self.add(self.data.len() - 1, v);
	}
	pub fn ask_prefix(&self, mut r: usize) -> A::Item {
		let mut res = self.alg.unit();
		while r != 0 {
			res = self.alg.op(self.data[r].clone(), res);
			r -= r.lsb();
		}
		res
	}
	// TODO: test
	pub fn partition_point<F: FnMut(A::Item) -> bool>(&self, mut pred: F) -> usize {
		let mut x = 0; // pred(&self.ask_prefix(x)) == true
		let mut w = (self.data.len() - 1).msb();
		let mut l = self.alg.unit();
		while w != 0 {
			if x + w < self.data.len() && pred(self.alg.op(l.clone(), self.data[x + w].clone()))
			{
				x += w;
				l = self.alg.op(l, self.data[x + w].clone());
			}
			w >>= 1;
		}
		x + 1
	}
	pub fn lower_bound(&self, v: A::Item) -> usize
	where
		A::Item: Ord,
	{
		self.partition_point(|x| x < v)
	}
	pub fn upper_bound(&self, v: A::Item) -> usize
	where
		A::Item: Ord,
	{
		self.partition_point(|x| x <= v)
	}
}

// A: Commutative
impl<A: Group> FenwickTree<A> {
	pub fn ask(&self, l: usize, r: usize) -> A::Item {
		self.alg.op(self.alg.inv(self.ask_prefix(l)), self.ask_prefix(r))
	}
}
