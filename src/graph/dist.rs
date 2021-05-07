pub use super::weighted::*;
use crate::assign::*;
use crate::bounded::*;
use crate::num::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra<G: WGraph>(g: &G, s: usize) -> Vec<G::W> where G::W: Num + Bounded {
    let mut dist = vec![G::W::MAX; g.len()];
    dist[s] = G::W::zero();
    let mut togo: BinaryHeap<_> = vec![Reverse((G::W::zero(), s))].into();
    while let Some(Reverse((d, v))) = togo.pop() {
        g.adj_w(v, |w, e| {
            if assign_if(d + e, &mut dist[w], |x, y| x < y) {
                togo.push(Reverse((d + e, w)));
            }
        });
    }
    dist
}
