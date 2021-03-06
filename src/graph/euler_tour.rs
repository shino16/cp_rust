pub use super::weighted::*;
pub use crate::alg::arith::*;

pub fn euler_tour<T: Copy, M: Group<T>, G: WGraph<T>>(
    g: &G,
    s: usize,
    alg: M,
) -> (Vec<T>, Vec<usize>, Vec<usize>, Vec<(usize, usize)>) {
    let mut edges = Vec::with_capacity(g.len() * 2);
    let mut togo = vec![(s, !0, alg.unit())];
    togo.reserve(g.len() * 2);
    let mut range = vec![(!0, !0); g.len()];
    let mut us = Vec::with_capacity(g.len() * 2);
    let mut vs = us.clone();
    while let Some((v, par, e)) = togo.pop() {
        if v > !v {
            range[!v].1 = edges.len();
            edges.push(alg.inv(e));
            us.push(!v);
            vs.push(par);
        } else {
            edges.push(e);
            us.push(par);
            vs.push(v);
            range[v].0 = edges.len();
            togo.push((!v, par, e));
            g.adj_w(v, |w, e| {
                if w != par {
                    togo.push((w, v, e));
                }
            });
        }
    }
    (edges, us, vs, range)
}
