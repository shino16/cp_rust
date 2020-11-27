pub use crate::alg::*;
use crate::bit::*;

#[derive(Clone)]
pub struct DisjointSparseTable<A: Alg> {
	data: Vec<Vec<A::Item>>,
	alg: A,
}

impl<A: Monoid> DisjointSparseTable<A> {
	pub fn new(data: Vec<A::Item>, alg: A) -> Self {
		let len = data.len();
		let height = len.ilog2() as usize;
		let mut data = vec![data; height + 1];
		for s in 1..=height {
			for z in (0..len).step_by(1 << (s + 1)) {
				let m = z + (1 << s);
				if m >= len {
					break;
				}
				data[s][m - 1] = data[0][m - 1].clone();
				data[s][m] = data[0][m].clone();
				for i in (z..m - 1).rev() {
					data[s][i] = alg.op(&data[0][i], &data[s][i + 1]);
				}
				for i in m + 1..(m + (1 << s)).min(len) {
					data[s][i] = alg.op(&data[s][i - 1], &data[0][i]);
				}
			}
		}
		Self { data, alg }
	}
	pub fn ask(&self, l: usize, r: usize) -> A::Item {
		if l == r {
			self.alg.unit()
		} else if l + 1 == r {
			self.data[0][l].clone()
		} else {
			let s = (l ^ r).ilog2() as usize;
			self.alg.op(&self.data[s][l], &self.data[s][r])
		}
	}
}
