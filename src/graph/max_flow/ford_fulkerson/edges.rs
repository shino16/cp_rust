pub use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Edge<C: Num + Bounded> {
	pub from: usize,
	pub to: usize,
	pub cap: C,
	pub flow: C,
}

impl<C: Num + Bounded> FordFulkerson<C> {
	pub fn get_edge(&self, v: usize, idx: usize) -> Edge<C> {
		let e = self.graph[v][idx];
		let rev = self.graph[e.to][e.rev];
		Edge { from: v, to: e.to, cap: e.cap + rev.cap, flow: rev.cap }
	}
	pub fn get_edges_from(&self, v: usize) -> Vec<Edge<C>> {
		(0..self.graph[v].len()).map(|idx| self.get_edge(v, idx)).collect()
	}
}
