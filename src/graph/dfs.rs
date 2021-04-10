pub use super::*;
use crate::ds::bitset::*;

pub mod cpnts;

pub fn dfs(g: &impl Graph, s: usize, mut f: impl FnMut(usize, usize)) {
    let mut visited = new_bitset(g.len());
    visited.set_bit(s);
    dfs_impl(g, s, !0, &mut visited, &mut f);
}

pub fn dfs_ord_par(g: &impl Graph, s: usize) -> (Vec<usize>, Vec<usize>) {
    let mut ord = Vec::with_capacity(g.len());
    let mut par = vec![!0; g.len()];
    dfs(g, s, |v, p| {
        ord.push(v);
        par[v] = p;
    });
    (ord, par)
}

pub fn dfs_all(g: &impl Graph, mut f: impl FnMut(usize, usize)) {
    let mut visited = new_bitset(g.len());
    for s in 0..g.len() {
        if visited.set_bit(s) {
            dfs_impl(g, s, !0, &mut visited, &mut f);
        }
    }
}

fn dfs_impl(
    g: &impl Graph,
    v: usize,
    par: usize,
    visited: &mut [u32],
    f: &mut impl FnMut(usize, usize),
) {
    f(v, par);
    g.adj(v, |w| {
        if visited.set_bit(w) {
            dfs_impl(g, w, v, visited, f);
        }
    });
}
