pub use crate::graph::weighted::*;

/// f: (v, par, w)
pub fn dfs<G: WGraph>(g: &G, s: usize, mut f: impl FnMut(usize, usize, G::W))
where
    G::W: Copy + Default,
{
    dfs_impl(g, s, !0, G::W::default(), &mut f);
}

fn dfs_impl<G: WGraph>(
    g: &G,
    v: usize,
    par: usize,
    w: G::W,
    f: &mut impl FnMut(usize, usize, G::W),
) where G::W: Copy {
    f(v, par, w);
    g.adj_w(v, |to, w| {
        if to != par {
            dfs_impl(g, to, v, w, f);
        }
    });
}
