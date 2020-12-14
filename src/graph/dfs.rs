pub use crate::graph::*;
use crate::ds::bitset::*;

// f: (v, par)
pub fn dfs<G: Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {
	let mut togo = vec![(s, !0)];
	let mut visited = new_bitset(g.len());
	visited.set_bit(s, true);
	while let Some((v, par)) = togo.pop() {
		f(v, par);
		g.adj(v, |w| {
			if visited.modify_bit(w, true) {
				togo.push((w, v));
			}
		});
	}
}
