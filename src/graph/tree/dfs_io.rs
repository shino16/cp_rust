pub use super::*;

#[derive(Debug)]
pub enum InOut {
    In(usize),
    Out(usize),
}

pub use InOut::*;

pub fn dfs_io(g: &impl Graph, s: usize, mut f: impl FnMut(InOut, usize)) {
    let mut togo = vec![(s, !0)];
    while let Some((v, par)) = togo.pop() {
        if v > !v {
            f(Out(!v), par);
        } else {
            f(In(v), par);
            togo.push((!v, par));
            g.adj(v, |w| {
                if w != par {
                    togo.push((w, v));
                }
            });
        }
    }
}
