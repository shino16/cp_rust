pub use super::*;
pub use crate::alg::arith::*;

pub fn euler_tour<A: Group, G: WTree<A::Item>>(g: &G, s: usize, alg: A) -> Vec<A::Item>
where
    A::Item: Clone,
{
    let mut edges = Vec::new();
    let mut togo = vec![(s, !0, alg.unit())];
    while let Some((v, par, e)) = togo.pop() {
        if v > !v {
            edges.push(alg.inv(e));
        } else {
            edges.push(e);
            togo.push((!v, 0, e));
            g.adj_w(v, |w, &e| {
                if w != par {
                    togo.push((w, v, e));
                }
            });
        }
    }
    edges
}
