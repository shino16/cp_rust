pub use super::*;

/// f: (v, par)
pub fn dfs<G: Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {
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

pub fn dfs_ord_par<G: Graph>(g: &G, s: usize) -> (Vec<usize>, Vec<usize>) {
    let mut ord = Vec::with_capacity(g.len());
    let mut par = vec![!0; g.len()];
    dfs(g, s, |v, p| {
        ord.push(v);
        par[v] = p;
    });
    (ord, par)
}
