pub use crate::alg::arith::*;

pub trait Cum {
	type Item: Copy;
	fn cuml<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>;
	fn cumr<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>;
	fn cuml_sum(&self) -> Vec<Self::Item>
	where
		Self::Item: Num,
	{
		self.cuml(Addition::new())
	}
	fn cumr_sum(&self) -> Vec<Self::Item>
	where
		Self::Item: Num,
	{
		self.cumr(Addition::new())
	}
}

impl<T: Copy> Cum for [T] {
	type Item = T;
	fn cuml<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item> {
		let mut res = Vec::with_capacity(self.len() + 1);
		let mut tl = m.unit();
		res.push(tl.clone());
		for e in self {
			tl = m.op(tl, e.clone());
			res.push(tl.clone());
		}
		res
	}

	fn cumr<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item> {
		let mut res = Vec::with_capacity(self.len() + 1);
		let mut tl = m.unit();
		res.push(tl.clone());
		for e in self.iter().rev() {
			tl = m.op(e.clone(), tl);
			res.push(tl.clone());
		}
		res.reverse();
		res
	}
}
