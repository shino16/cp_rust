pub use crate::graph::*;
#[derive(Debug, Default)]
pub struct DiGraph(Vec<Vec<usize>>);

impl<'g> Graph<'g> for DiGraph {
	type V = usize;
	type E = ();
	type VIter = std::iter::Copied<std::slice::Iter<'g, Self::V>>;
	type EIter = std::iter::Take<std::iter::Repeat<()>>;
	fn add_edge(&mut self, u: Self::V, v: Self::V, (): Self::E) {
		self.0[u].push(v);
	}
	fn adj_v<'a: 'g>(&'a self, v: Self::V) -> Self::VIter {
		self.0[v].iter().copied()
	}
	fn adj_e<'a: 'g>(&'a self, v: Self::V) -> Self::EIter {
		std::iter::repeat(()).take(self.0[v].len())
	}
	fn dim(&self) -> Self::V {
		self.0.len()
	}
}

impl DiGraph {
	pub fn new(n: usize) -> Self {
		Self(vec![Vec::new(); n])
	}
}

#[derive(Debug, Default)]
pub struct BiDir<G>(G);

impl<'g, G: Graph<'g>> Graph<'g> for BiDir<G> {
	type V = G::V;
	type E = G::E;
	type VIter = G::VIter;
	type EIter = G::EIter;
	fn add_edge<'a>(&'a mut self, u: Self::V, v: Self::V, e: Self::E) {
		self.0.add_edge(u, v, e);
		self.0.add_edge(v, u, e);
	}
	fn adj_v<'a: 'g>(&'a self, v: Self::V) -> Self::VIter {
		self.0.adj_v(v)
	}
	fn adj_e<'a: 'g>(&'a self, v: Self::V) -> Self::EIter {
		self.0.adj_e(v)
	}
	fn dim(&self) -> Self::V {
		self.0.dim()
	}
}

pub type UGraph = BiDir<DiGraph>;

impl UGraph {
	pub fn new(n: usize) -> Self {
		Self(DiGraph::new(n))
	}
}
