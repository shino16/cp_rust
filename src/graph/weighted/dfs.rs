pub use super::*;
use crate::ds::bitset::*;

/// f: (v, par, w)
pub fn dfs<W: Copy + Default>(
    g: &impl WGraph<W>,
    s: usize,
    mut f: impl FnMut(usize, usize, W),
) {
    let mut visited = new_bitset(g.len());
    visited.set_bit(s);
    dfs_impl(g, s, !0, W::default(), &mut visited, &mut f);
}

fn dfs_impl<W: Copy>(
    g: &impl WGraph<W>,
    v: usize,
    par: usize,
    w: W,
    visited: &mut [u32],
    f: &mut impl FnMut(usize, usize, W),
) {
    f(v, par, w);
    g.adj_w(v, |to, w| {
        if visited.set_bit(to) {
            dfs_impl(g, to, v, w, visited, f);
        }
    });
}
