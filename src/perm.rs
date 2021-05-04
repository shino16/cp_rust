pub fn perm_inv(p: &[usize]) -> Vec<usize> {
    let mut res = vec![!0; p.len()];
    for i in 0..p.len() {
        res[p[i]] = i;
    }
    res
}
