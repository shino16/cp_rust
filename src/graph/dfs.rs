pub use super::*;
use crate::ds::bitset::*;

/// f: (v, par)
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

pub fn is_connected<G: Graph>(g: &G) -> bool {
	let mut cnt = 0;
	dfs(g, 0, |_, _| cnt += 1);
	cnt == g.len()
}

pub fn dfs_ord_par<G: Graph>(g: &G, s: usize) -> (Vec<(usize, usize)>, Vec<usize>) {
	let mut ord = Vec::with_capacity(g.len());
	let mut par = vec![!0; g.len()];
	dfs(g, s, |v, p| {
		ord.push((v, p));
		par[v] = p;
	});
	(ord, par)
}
