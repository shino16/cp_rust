pub use super::*;
use crate::zo::ZeroOne;

pub trait WGraph<W: Copy>: Graph {
    fn adj_w<F: FnMut(usize, W)>(&self, v: usize, f: F);
}
impl<T: ZeroOne> WGraph<T> for Vec<Vec<usize>> {
    fn adj_w<F: FnMut(usize, T)>(&self, v: usize, mut f: F) {
        self[v].iter().for_each(|&v| f(v, T::ONE));
    }
}
impl<W: Copy> WGraph<W> for Vec<Vec<(usize, W)>> {
    fn adj_w<F: FnMut(usize, W)>(&self, v: usize, mut f: F) {
        self[v].iter().for_each(|&(v, w)| f(v, w));
    }
}
