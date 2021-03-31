pub use super::*;
use crate::ds::bitset::*;

#[derive(Debug)]
pub enum InOut {
    In(usize),
    Out(usize),
}

pub use InOut::*;

pub fn dfs_io(g: &impl Graph, s: usize, mut f: impl FnMut(InOut, usize)) {
    fn dfs_impl(
        g: &impl Graph,
        v: usize,
        par: usize,
        visited: &mut [u32],
        f: &mut impl FnMut(InOut, usize),
    ) {
        f(In(v), par);
        g.adj(v, |w| {
            if visited.set_bit(w) {
                dfs_impl(g, w, v, visited, f);
            }
        });
        f(Out(v), par);
    }
    let mut visited = new_bitset(g.len());
    visited.set_bit(s);
    dfs_impl(g, s, !0, &mut visited, &mut f);
}
