pub mod dfs;
pub mod dfs_io;
pub mod dijkstra;
pub mod euler_tour;

use crate::int::ZeroOne;

pub trait Graph {
	fn len(&self) -> usize;
	fn adj<F: FnMut(usize)>(&self, v: usize, f: F);
}

pub trait WGraph<W>: Graph {
	fn adj_w<F: FnMut(usize, &W)>(&self, v: usize, f: F);
}

pub trait Tree: Graph {}

pub trait WTree<W>: WGraph<W> {}

impl Graph for Vec<Vec<usize>> {
	fn len(&self) -> usize {
		self.len()
	}
	fn adj<F: FnMut(usize)>(&self, v: usize, f: F) {
		self[v].iter().copied().for_each(f);
	}
}

impl<N: ZeroOne> WGraph<N> for Vec<Vec<usize>> {
	fn adj_w<F: FnMut(usize, &N)>(&self, v: usize, mut f: F) {
		self[v].iter().for_each(|&v| f(v, &N::ONE))
	}
}

impl<W> Graph for Vec<Vec<(usize, W)>> {
	fn len(&self) -> usize {
		self.len()
	}
	fn adj<F: FnMut(usize)>(&self, v: usize, mut f: F) {
		self[v].iter().for_each(|&(v, _)| f(v))
	}
}

impl<W> WGraph<W> for Vec<Vec<(usize, W)>> {
	fn adj_w<F: FnMut(usize, &W)>(&self, v: usize, mut f: F) {
		self[v].iter().for_each(|&(v, ref e)| f(v, e))
	}
}

impl<G: Graph> Tree for G {}
impl<W, G: WGraph<W>> WTree<W> for G {}
