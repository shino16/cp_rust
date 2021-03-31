pub use super::*;

pub fn topological_sort(g: &impl Graph) -> Option<Vec<usize>> {
    let mut in_deg = vec![0; g.len()];
    for v in 0..g.len() {
        g.adj(v, |w| in_deg[w] += 1);
    }
    let mut stack = Vec::with_capacity(g.len());
    for v in 0..g.len() {
        if in_deg[v] == 0 {
            stack.push(v);
        }
    }
    let mut res = Vec::with_capacity(g.len());
    while let Some(v) = stack.pop() {
        res.push(v);
        g.adj(v, |w| {
            in_deg[w] -= 1;
            if in_deg[w] == 0 {
                stack.push(w);
            }
        });
    }
    if res.len() == g.len() { Some(res) } else { None }
}
