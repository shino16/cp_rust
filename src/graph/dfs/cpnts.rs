pub use super::*;

/// dfs-ord
pub fn components(g: &impl Graph) -> Vec<Vec<usize>> {
    let mut visited = new_bitset(g.len());
    let mut groups = Vec::new();
    for v in 0..g.len() {
        if visited.set_bit(v) {
            let mut group = Vec::new();
            dfs_impl(g, v, !0, &mut visited, &mut |v, _| {
                group.push(v);
            });
            groups.push(group);
        }
    }
    groups
}
