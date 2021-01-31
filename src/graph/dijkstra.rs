pub use super::*;
use crate::assign::*;
use crate::int::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra<I: UInt, G: WGraph<I>>(g: &G, s: usize) -> Vec<I> {
	let mut dist = vec![I::MAX; g.len()];
	dist[s] = I::ZERO;
	let mut togo: BinaryHeap<_> = vec![Reverse((I::ZERO, s))].into();
	while let Some(Reverse((d, v))) = togo.pop() {
		g.adj_w(v, |w, &e| {
			if assign_if(d + e, &mut dist[w], |x, y| x < y) {
				togo.push(Reverse((d + e, w)));
			}
		});
	}
	dist
}
