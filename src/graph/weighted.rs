pub use super::*;

pub trait WGraph: Graph {
    type W;
    fn adj_w(&self, v: usize, f: impl FnMut(usize, Self::W));
}
impl WGraph for Vec<Vec<usize>> {
    type W = usize;
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, Self::W)) {
        self[v].iter().for_each(|&v| f(v, 1));
    }
}
impl<W: Copy> WGraph for Vec<Vec<(usize, W)>> {
    type W = W;
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, W)) {
        self[v].iter().for_each(|&(v, w)| f(v, w));
    }
}
