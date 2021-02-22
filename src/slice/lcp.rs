pub fn lcp(t: &[u8], sa: &[usize]) -> Vec<usize> {
    lcp_impl(t, sa, |v| v as usize)
}

pub fn lcp_impl<T: Copy, F: FnMut(T) -> usize>(t: &[T], sa: &[usize], mut key: F) -> Vec<usize> {
    let n = sa.len() - 1;
    let mut rank = vec![0; n];
    for i in 1..n {
        rank[sa[i]] = i;
    }
    let mut k = 0;
    let mut lcp = vec![0; n - 1];
    for i in 0..n {
        if rank[i] == n - 1 {
            k = 0;
            continue;
        }
        let j = sa[rank[i] + 1];
        while i + k < n && j + k < n && key(t[i + k]) == key(t[j + k]) {
            k += 1;
        }
        lcp[rank[i]] = k;
        if k > 0 {
            k -= 1;
        }
    }
    lcp
}
