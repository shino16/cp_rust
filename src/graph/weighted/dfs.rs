pub use super::*;
use crate::ds::bitset::*;

/// f: (v, par, w)
pub fn dfs<W: Copy + Default, G: WGraph<W>, F: FnMut(usize, usize, W)>(
    g: &G,
    s: usize,
    mut f: F,
) {
    let mut visited = new_bitset(g.len());
    visited.set_bit(s);
    _dfs_impl(g, s, !0, W::default(), &mut visited, &mut f);
}

fn _dfs_impl<W: Copy, G: WGraph<W>, F: FnMut(usize, usize, W)>(
    g: &G,
    v: usize,
    par: usize,
    w: W,
    visited: &mut [u32],
    f: &mut F,
) {
    f(v, par, w);
    g.adj_w(v, |to, w| {
        if visited.set_bit(to) {
            _dfs_impl(g, to, v, w, visited, f);
        }
    });
}
