pub use super::*;
use crate::ds::bitset::*;

#[derive(Debug)]
pub enum InOut {
    In(usize),
    Out(usize),
}

pub use InOut::*;

pub fn dfs_io<G: Graph, F: FnMut(InOut, usize)>(g: &G, s: usize, mut f: F) {
    fn dfs_impl<G: Graph, F: FnMut(InOut, usize)>(
        g: &G,
        v: usize,
        par: usize,
        visited: &mut [u32],
        f: &mut F,
    ) {
        f(In(v), par);
        g.adj(v, |w| {
            if visited.modify_bit(w, true) {
                dfs_impl(g, w, v, visited, f);
            }
        });
        f(Out(v), par);
    }
    let mut visited = new_bitset(g.len());
    visited.set_bit(s, true);
    dfs_impl(g, s, !0, &mut visited, &mut f);
}
