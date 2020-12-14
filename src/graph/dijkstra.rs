pub use crate::graph::*;
use crate::int::*;
use crate::ord::*;

pub fn dijkstra<I: UInt, G: WGraph<I>>(g: &G, s: usize) -> Vec<I> {
	let mut dist = vec![I::MAX; g.len()];
	dist[s] = I::ZERO;
	let mut togo = vec![s];
	while let Some(v) = togo.pop() {
		g.adj_w(v, |w, &e| {
			let d = dist[v] + e;
			if dist[w].chmin(d) {
				togo.push(w);
			}
		});
	}
	dist
}
