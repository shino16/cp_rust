pub mod bf;
pub mod bfs;
pub mod dfs;
pub mod dfs_io;
pub mod dist;
pub mod grid;
pub mod io;
pub mod max_flow;
pub mod tree;
pub mod tsort;
pub mod weighted;

pub trait Graph {
    fn len(&self) -> usize;
    fn adj(&self, v: usize, f: impl FnMut(usize));
}
impl Graph for Vec<Vec<usize>> {
    fn len(&self) -> usize { self.len() }
    fn adj(&self, v: usize, f: impl FnMut(usize)) {
        self[v].iter().copied().for_each(f);
    }
}
