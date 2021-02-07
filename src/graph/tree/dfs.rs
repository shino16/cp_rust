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

pub fn topological<G: Graph>(g: &G, s: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::with_capacity(g.len());
    dfs(g, s, |v, par| res.push((v, par)));
    res
}
