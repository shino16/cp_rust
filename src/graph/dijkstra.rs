pub use super::weighted::*;
use crate::assign::*;
use crate::int::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra<G: WGraph>(g: &G, s: usize) -> Vec<G::W> where G::W: Int {
    let mut dist = vec![G::W::MAX; g.len()];
    dist[s] = G::W::ZERO;
    let mut togo: BinaryHeap<_> = vec![Reverse((G::W::ZERO, s))].into();
    while let Some(Reverse((d, v))) = togo.pop() {
        g.adj_w(v, |w, e| {
            if assign_if(d + e, &mut dist[w], |x, y| x < y) {
                togo.push(Reverse((d + e, w)));
            }
        });
    }
    dist
}
