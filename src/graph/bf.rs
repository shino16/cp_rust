pub use super::weighted::*;
pub use crate::bounded::*;
use crate::num::*;

pub fn bellman_ford<G: WGraph>(g: &G, s: usize) -> Option<Vec<G::W>> where G::W: INum + Bounded {
    let mut d = vec![G::W::MAX; g.len()];
    d[s] = G::W::ZERO;
    for _ in 0..g.len() {
        let mut done = false;
        for v in 0..g.len() {
            if d[v] == G::W::MAX {
                continue;
            }
            g.adj_w(v, |to, w| if d[to] > d[v] + w {
                d[to] = d[v] + w;
                done = true;
            });
        }
        if !done {
            return Some(d);
        }
    }
    None
}
