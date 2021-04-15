pub use super::dfs::weighted::*;
use crate::num::*;

pub fn dist<G: WGraph>(g: &G, s: usize) -> Vec<G::W> where G::W: Num + Default {
    let mut dist = vec![G::W::ZERO; g.len()];
    dfs(g, s, |v, p, w| if p != !0 {
        dist[v] = dist[p] + w;
    });
    dist
}
