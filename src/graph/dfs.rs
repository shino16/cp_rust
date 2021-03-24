pub use super::*;
use crate::ds::bitset::*;

pub mod cpnts;

/// f: (v, par)
pub fn dfs<G: Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {
    let mut visited = new_bitset(g.len());
    visited.set_bit(s);
    _dfs_impl(g, s, !0, &mut visited, &mut f);
}

pub fn dfs_ord_par<G: Graph>(g: &G, s: usize) -> (Vec<usize>, Vec<usize>) {
    let mut ord = Vec::with_capacity(g.len());
    let mut par = vec![!0; g.len()];
    dfs(g, s, |v, p| {
        ord.push(v);
        par[v] = p;
    });
    (ord, par)
}

fn _dfs_impl<G: Graph, F: FnMut(usize, usize)>(
    g: &G,
    v: usize,
    par: usize,
    visited: &mut [u32],
    f: &mut F,
) {
    f(v, par);
    g.adj(v, |w| {
        if visited.set_bit(w) {
            _dfs_impl(g, w, v, visited, f);
        }
    });
}
