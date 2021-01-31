pub use super::*;
use crate::ds::bitset::*;

#[derive(Debug)]
pub enum InOut {
	In(usize),
	Out(usize),
}

pub use InOut::*;

pub fn dfs_io<G: Graph, F: FnMut(InOut, usize)>(g: &G, s: usize, mut f: F) {
	let mut visited = new_bitset(g.len());
	visited.set_bit(s, true);
	let mut togo = vec![(s, !0)];
	while let Some((v, par)) = togo.pop() {
		if v >> 31 != 0 {
			f(Out(!v), par);
		} else {
			f(In(v), par);
			togo.push((!v, par));
			g.adj(v, |w| {
				if visited.modify_bit(w, true) {
					togo.push((w, v));
				}
			});
		}
	}
}
