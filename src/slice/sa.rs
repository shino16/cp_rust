use std::cmp::Reverse;

pub struct SuffixArray<T> {
    _data: Vec<T>,
    sa: Vec<usize>,
    rev: Vec<usize>,
}

impl<T: Copy> SuffixArray<T> {
    pub fn new<F: FnMut(T) -> usize>(mut _data: Vec<T>, zero: T, mut key: F) -> Self {
        _data.push(zero);
        let n = _data.len();
        let mut sa: Vec<_> = (0..n).collect();
        sa.sort_unstable_by_key(|&i| (key(_data[i]), Reverse(i)));
        let mut c: Vec<_> = _data.iter().copied().map(key).collect();
        let mut c2 = vec![0; n];
        let mut cnt = vec![0; n];
        let mut len = 1;
        while len < n {
            for i in 0..n {
                let cond = i != 0
                    && c[sa[i - 1]] == c[sa[i]]
                    && sa[i - 1] + len < n
                    && c[sa[i - 1] + len / 2] == c[sa[i] + len / 2];
                c2[sa[i]] = if cond { c2[sa[i - 1]] } else { 1 };
            }
            cnt.iter_mut().for_each(|e| *e = 0);
            c.clear();
            c.extend(sa.iter().copied());
            for i in 0..n {
                if let Some(s) = c[i].checked_sub(len) {
                    sa[cnt[c2[s]]] = s;
                    cnt[c2[s]] += 1;
                }
            }
            std::mem::swap(&mut c2, &mut c);
            len *= 2;
        }
        let mut rev = vec![!0; n];
        for i in 0..n {
            rev[sa[i]] = i;
        }
        Self { _data, sa, rev }
    }
    pub fn nth(&self, i: usize) -> usize { self.sa[i] }
    pub fn rank(&self, i: usize) -> usize { self.rev[i] }
}

impl<T> std::ops::Deref for SuffixArray<T> {
    type Target = [usize];
    fn deref(&self) -> &Self::Target {
        &self.sa[..]
    }
}
