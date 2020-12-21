pub use super::*;
use crate::assign::*;
use crate::int::*;

pub fn dijkstra<I: UInt, G: WGraph<I>>(g: &G, s: usize) -> Vec<I> {
	let mut dist = vec![I::MAX; g.len()];
	dist[s] = I::ZERO;
	let mut togo = vec![s];
	while let Some(v) = togo.pop() {
		g.adj_w(v, |w, &e| {
			if assign_if(dist[v] + e, &mut dist[w], |x, y| x < y) {
				togo.push(w);
			}
		});
	}
	dist
}
