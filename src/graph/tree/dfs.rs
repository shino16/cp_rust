pub use super::*;

pub mod weighted;

/// f: (v, par)
pub fn dfs(g: &impl Graph, s: usize, mut f: impl FnMut(usize, usize)) {
    let mut togo = vec![(s, !0)];
    while let Some((v, par)) = togo.pop() {
        f(v, par);
        g.adj(v, |w| {
            if w != par {
                togo.push((w, v));
            }
        });
    }
}

pub fn dfs_ord_par(g: &impl Graph, s: usize) -> (Vec<usize>, Vec<usize>) {
    let mut ord = Vec::with_capacity(g.len());
    let mut par = vec![!0; g.len()];
    dfs(g, s, |v, p| {
        ord.push(v);
        par[v] = p;
    });
    (ord, par)
}
