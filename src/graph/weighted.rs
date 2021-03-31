pub use super::*;
use crate::zo::ZeroOne;

pub mod dfs;

pub trait WGraph<W>: Graph {
    fn adj_w(&self, v: usize, f: impl FnMut(usize, W));
}
impl<T: ZeroOne> WGraph<T> for Vec<Vec<usize>> {
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, T)) {
        self[v].iter().for_each(|&v| f(v, T::ONE));
    }
}
impl WGraph<()> for Vec<Vec<usize>> {
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, ())) {
        self[v].iter().for_each(|&v| f(v, ()));
    }
}
impl<W: Copy> WGraph<W> for Vec<Vec<(usize, W)>> {
    fn adj_w(&self, v: usize, mut f: impl FnMut(usize, W)) {
        self[v].iter().for_each(|&(v, w)| f(v, w));
    }
}
