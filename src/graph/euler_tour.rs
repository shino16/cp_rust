pub use super::weighted::*;
pub use crate::alg::arith::*;

pub fn euler_tour<T: Copy, M: Group<T>, G: WGraph<T>>(g: &G, s: usize, alg: M) -> Vec<T> {
    let mut edges = Vec::new();
    let mut togo = vec![(s, !0, alg.unit())];
    while let Some((v, par, e)) = togo.pop() {
        if v > !v {
            edges.push(alg.inv(e));
        } else {
            edges.push(e);
            togo.push((!v, 0, e));
            g.adj_w(v, |w, e| {
                if w != par {
                    togo.push((w, v, e));
                }
            });
        }
    }
    edges
}
