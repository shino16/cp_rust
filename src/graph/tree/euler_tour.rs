pub use crate::alg::arith::*;
pub use crate::graph::weighted::*;

pub fn euler_tour<G: WGraph>(
    g: &G,
    s: usize,
    alg: impl Group<G::W>,
) -> (Vec<G::W>, Vec<usize>, Vec<usize>, Vec<(usize, usize)>)
where
    G::W: Copy,
{
    let mut edges = Vec::with_capacity(g.len() * 2);
    let mut togo = Vec::with_capacity(g.len() * 2);
    g.adj_w(s, |to, e| togo.push((to, s, e)));
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
    range[s] = (0, edges.len());
    (edges, us, vs, range)
}
