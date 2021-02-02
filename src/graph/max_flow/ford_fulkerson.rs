use crate::bounded::Bounded;
use crate::ds::bitset::*;
pub use crate::num::*;

pub mod edge;

#[derive(Clone, Copy, Debug)]
pub struct Edge<C: Num + Bounded> {
	pub to: usize,
	pub cap: C,
	rev: usize,
}

/// O(FE)
#[derive(Clone)]
pub struct FordFulkerson<C: Num + Bounded> {
	pub graph: Vec<Vec<Edge<C>>>,
}

impl<C: Num + Bounded> FordFulkerson<C> {
	pub fn new(len: usize) -> Self {
		Self { graph: vec![Vec::new(); len] }
	}
	pub fn len(&self) -> usize {
		self.graph.len()
	}
	pub fn from_digraph(graph: &[Vec<(usize, C)>]) -> Self {
		let mut ret = Self::new(graph.len());
		for (v, adj) in (0..).zip(graph) {
			for &(w, cap) in adj {
				ret.add_edge(v, w, cap);
			}
		}
		ret
	}
	pub fn add_vertex(&mut self) -> usize {
		self.graph.push(Vec::new());
		self.graph.len() - 1
	}
	pub fn add_edge(&mut self, v: usize, w: usize, cap: C) {
		let (vidx, widx) = (self.graph[v].len(), self.graph[w].len());
		self.graph[v].push(Edge { to: w, cap, rev: widx });
		self.graph[w].push(Edge { to: v, cap: C::ZERO, rev: vidx });
	}
	pub fn solve(&mut self, s: usize, t: usize) -> C {
		let mut res = C::ZERO;
		let mut used = new_bitset(self.graph.len());
		loop {
			used.reset();
			let f = Self::dfs(&mut self.graph, s, t, &mut used, C::MAX);
			if f == C::ZERO {
				return res;
			}
			res += f;
		}
	}
	fn dfs(graph: &mut Vec<Vec<Edge<C>>>, v: usize, t: usize, used: &mut [u32], ub: C) -> C {
		if v == t {
			return ub;
		}
		let mut adj = std::mem::take(&mut graph[v]);
		for &mut Edge { to, ref mut cap, rev } in &mut adj {
			if *cap != C::ZERO && used.modify_bit(to, true) {
				let df = Self::dfs(graph, to, t, used, ub.min(*cap));
				if df != C::ZERO {
					*cap -= df;
					graph[to][rev].cap += df;
					graph[v] = adj;
					return df;
				}
			}
		}
		graph[v] = adj;
		C::ZERO
	}
}
