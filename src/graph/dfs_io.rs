pub use crate::ds::bitset::*;
use crate::graph::*;

pub fn dfs_io<G: Graph, FI: FnMut(usize, usize), FO: FnMut(usize, usize)>(
	g: &G,
	s: usize,
	mut fi: FI,
	mut fo: FO,
) {
	let mut visited = new_bitset(g.len());
	visited.set_bit(s, true);
	let mut togo = vec![(s, !0)];
	while let Some((v, par)) = togo.pop() {
		if v.get_bit(31) {
			fo(!v, par);
		} else {
			fi(v, par);
			togo.push((!v, par));
			g.adj(v, |w| {
				if visited.modify_bit(w, true) {
					togo.push((w, v));
				}
			});
		}
	}
}
