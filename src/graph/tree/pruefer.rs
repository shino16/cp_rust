pub fn decode(code: &[usize]) -> Vec<(usize, usize)> {
    let n = code.len() + 2;
    let mut deg = vec![1; n];
    for &v in code {
        deg[v] += 1;
    }
    let mut i = 0;
    while deg[i] != 1 {
        i += 1;
    }
    let mut leaf = i;
    let mut edges = Vec::with_capacity(n - 1);
    for &v in code {
        edges.push((leaf, v));
        deg[v] -= 1;
        if deg[v] == 1 && v < i {
            leaf = v;
        } else {
            i += 1;
            while deg[i] != 1 {
                i += 1;
            }
            leaf = i;
        }
    }
    edges.push((leaf, n - 1));
    edges
}
