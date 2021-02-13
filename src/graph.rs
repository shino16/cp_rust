pub mod bfs;
pub mod dfs;
pub mod dfs_io;
pub mod dijkstra;
pub mod euler_tour;
pub mod grid;
pub mod io;
pub mod max_flow;
pub mod tree;
pub mod weighted;

pub trait Graph {
    fn len(&self) -> usize;
    fn adj<F: FnMut(usize)>(&self, v: usize, f: F);
}
impl Graph for Vec<Vec<usize>> {
    fn len(&self) -> usize { self.len() }
    fn adj<F: FnMut(usize)>(&self, v: usize, f: F) { self[v].iter().copied().for_each(f); }
}
impl<W> Graph for Vec<Vec<(usize, W)>> {
    fn len(&self) -> usize { self.len() }
    fn adj<F: FnMut(usize)>(&self, v: usize, mut f: F) { self[v].iter().for_each(|&(v, _)| f(v)) }
}
