pub type Int = usize;

/// return (f(x), x) where f(x) is (locally) minimal
// verification: https://codeforces.com/contest/1479/submission/109458067
pub fn convex_min<T: Ord>(
    mut l: Int,
    mut r: Int,
    mut f: impl FnMut(Int) -> T,
) -> (T, Int) {
    r -= 1;
    // f(r) < f(r + 1)
    while l != r {
        let m = (l + r) / 2;
        if f(m) < f(m + 1) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    (f(l), l)
}
