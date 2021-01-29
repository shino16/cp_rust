// use crate::bound::Bound;
// pub use crate::num::*;

// #[derive(Clone, Copy, Debug)]
// pub struct Edge<C: INum + Bound> {
// 	pub to: usize,
// 	pub cap: C,
// 	rev: usize,
// }

// /// O(V^2E)
// #[derive(Clone)]
// pub struct PushRelabel<C: INum + Bound> {
// 	pub graph: Vec<Vec<Edge<C>>>,
// }

// impl<C: INum + Bound> PushRelabel<C> {
// 	pub fn new(len: usize) -> Self {
// 		Self { graph: vec![Vec::new(); len] }
// 	}
// 	pub fn len(&self) -> usize {
// 		self.graph.len()
// 	}
// 	pub fn from_digraph(graph: &[Vec<(usize, C)>]) -> Self {
// 		let mut ret = Self::new(graph.len());
// 		for (v, adj) in (0..).zip(graph) {
// 			for &(w, cap) in adj {
// 				ret.add_edge(v, w, cap);
// 			}
// 		}
// 		ret
// 	}
// 	pub fn add_vertex(&mut self) -> usize {
// 		self.graph.push(Vec::new());
// 		self.graph.len() - 1
// 	}
// 	pub fn add_edge(&mut self, v: usize, w: usize, cap: C) {
// 		let (vidx, widx) = (self.graph[v].len(), self.graph[w].len());
// 		self.graph[v].push(Edge { to: w, cap, rev: widx });
// 		self.graph[w].push(Edge { to: v, cap: C::ZERO, rev: vidx });
// 	}
// 	pub fn solve(&mut self, s: usize, t: usize) -> C {
// 		let mut height = vec![0; self.len()];
// 		height[s] = self.len();
// 		let mut excess = vec![C::ZERO; self.len()];
// 		excess[s] = C::MAX;
// 		excess[t] = -C::MAX;

// 		let push = |v: usize, e: &mut Edge<C>| {

// 		};

// 		panic!("{}", "")
// 	}
// }
