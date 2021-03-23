pub use super::*;

/// dfs-ord
pub fn components<G: Graph>(g: &G) -> Vec<Vec<usize>> {
    let mut visited = new_bitset(g.len());
    let mut groups = Vec::new();
    for v in 0..g.len() {
        let mut group = Vec::new();
        visited.set_bit(v);
        _dfs_impl(g, v, !0, &mut visited, &mut |v, _| {
            group.push(v);
        });
        groups.push(group);
    }
    groups
}
