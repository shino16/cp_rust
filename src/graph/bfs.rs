use std::collections::VecDeque;

pub use super::*;
use crate::ds::bitset::*;

/// f: (v, par)
pub fn bfs(g: &impl Graph, s: usize, mut f: impl FnMut(usize, usize)) {
    let mut visited = new_bitset(g.len());
    let mut togo: VecDeque<_> = vec![s].into();
    visited.set_bit(s);
    f(s, !0);
    while let Some(v) = togo.pop_front() {
        g.adj(v, |w| {
            if visited.set_bit(w) {
                f(w, v);
                togo.push_back(w);
            }
        });
    }
}
