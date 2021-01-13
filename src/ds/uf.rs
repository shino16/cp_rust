#[derive(Clone)]
pub struct UnionFind {
	par: Vec<usize>,
	size: Vec<usize>,
	count: usize,
}

impl UnionFind {
	pub fn new(len: usize) -> Self {
		let par: Vec<_> = (0..len).collect();
		let size = vec![1; len];
		Self { par, size, count: len }
	}
	pub fn len(&self) -> usize {
		self.par.len()
	}
	pub fn find(&mut self, x: usize) -> usize {
		if self.par[x] == x {
			x
		} else {
			self.par[x] = self.find(self.par[x]);
			self.par[x]
		}
	}
	pub fn is_same(&mut self, x: usize, y: usize) -> bool {
		self.find(x) == self.find(y)
	}
	pub fn size(&mut self, x: usize) -> usize {
		let root = self.find(x);
		self.size[root]
	}
	pub fn unite(&mut self, x: usize, y: usize) -> usize {
		let (mut x, mut y) = (self.find(x), self.find(y));
		if x != y {
			if self.size[x] < self.size[y] {
				std::mem::swap(&mut x, &mut y);
			}
			self.par[y] = x;
			self.size[x] += self.size[y];
			self.count -= 1;
		}
		x
	}
	pub fn count(&self) -> usize {
		self.count
	}
	pub fn push(&mut self) -> usize {
		let new = self.len();
		self.par.push(new);
		self.size.push(1);
		new
	}
	pub fn group(&self) -> Vec<Vec<usize>> {
		let mut groups = vec![Vec::new(); self.len()];
		for (i, &x) in (0..).zip(&self.par) {
			groups[x].push(i);
		}
		groups.retain(|v| !v.is_empty());
		groups
	}
}
