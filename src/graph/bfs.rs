use std::collections::VecDeque;

pub use super::*;
use crate::ds::bitset::*;

/// f: (v, par)
pub fn bfs<G: Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {
	let mut visited = new_bitset(g.len());
	let mut togo: VecDeque<_> = vec![(s, !0)].into();
	visited.set_bit(s, true);
	while let Some((v, par)) = togo.pop_front() {
		f(v, par);
		g.adj(v, |w| {
			if visited.modify_bit(w, true) {
				togo.push_back((w, v));
			}
		})
	}
}
