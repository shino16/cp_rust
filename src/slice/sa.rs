use super::sort::*;

pub fn suffix_array(v: &mut Vec<u8>) -> Vec<usize> {
    let mut out = Vec::with_capacity(v.len());
    v.extend_from_slice(&[0; 3]);
    suffix_array_impl(&v, &mut out, !0_u8 as usize, |v| v as usize);
    out
}

// reference
// Kärkkäinen, Sanders and Burkhardt
/// require exactly 3 sentinels in the last
pub fn suffix_array_impl<T: Copy, F: FnMut(T) -> usize>(
    t: &[T],
    out: &mut Vec<usize>,
    max_key: usize,
    mut key: F,
) {
    let n = t.len() - 3;

    out.clear();
    out.reserve(n + 1);
    out.push(n);

    if n == 0 {
        return;
    } else if n == 1 {
        out.push(1);
        return;
    }
    let (n0, n1, n2) = ((n + 2) / 3, (n + 1) / 3, n / 3);
    let n02 = n0 + n2;
    let (mut r, mut sa12) = (Vec::with_capacity(n02 + 3), Vec::with_capacity(n02));
    for i in 0..n + (n0 - n1) {
        if i % 3 != 0 {
            r.push(i);
        }
    }
    debug_assert_eq!(r.len(), n02);

    // radix sort [(t[i], t[i + 1], t[i + 2]) | i % 3 != 0]
    count_sort(&r, &mut sa12, max_key, |v| key(t[v + 2]));
    count_sort(&sa12, &mut r, max_key, |v| key(t[v + 1]));
    count_sort(&r, &mut sa12, max_key, |v| key(t[v]));

    let (mut name, mut c0, mut c1, mut c2) = (0, !0, !0, !0);
    for &i in &sa12 {
        if key(t[i]) != c0 || key(t[i + 1]) != c1 || key(t[i + 2]) != c2 {
            name += 1;
            c0 = key(t[i]);
            c1 = key(t[i + 1]);
            c2 = key(t[i + 2]);
        }
        if i % 3 == 1 {
            r[i / 3] = name;
        } else {
            r[i / 3 + n0] = name;
        }
    }
    if name < n02 {
        r.extend_from_slice(&[0; 3]);
        suffix_array_impl(&r, &mut sa12, name, std::convert::identity);
        for (name, &i) in (1..).zip(&sa12) {
            r[i] = name;
        }
    } else {
        for (i, &name) in (0..).zip(&r) {
            sa12[name - 1] = i;
        }
    }

    let (mut r0, mut sa0) = (Vec::with_capacity(n0), Vec::with_capacity(n0));
    for &i in &sa12 {
        if i < n0 {
            r0.push(i * 3);
        }
    }
    count_sort(&r0, &mut sa0, max_key, |v| key(t[v]));

    // sentinel
    r.extend_from_slice(&[0; 3]);
    let enc = |i| {
        if i < n0 { i * 3 + 1 } else { (i - n0) * 3 + 2 }
    };
    let mut p = sa0.into_iter();
    let mut q = sa12.into_iter().skip(n0 - n1);
    let (mut pi, mut qi) = (p.next().unwrap(), q.next().unwrap());
    loop {
        let i = enc(qi);
        let j = pi;
        let cond = if qi < n0 {
            (key(t[i]), r[qi + n0]) <= (key(t[j]), r[j / 3])
        } else {
            (key(t[i]), key(t[i + 1]), r[qi - n0 + 1])
                <= (key(t[j]), key(t[j + 1]), r[j / 3 + n0])
        };
        if cond {
            out.push(i);
            if let Some(q) = q.next() {
                qi = q;
            } else {
                out.push(j);
                out.extend(p);
                assert_eq!(out.len(), n + 1);
                return;
            }
        } else {
            out.push(j);
            if let Some(p) = p.next() {
                pi = p;
            } else {
                out.push(i);
                out.extend(q.map(enc));
                assert_eq!(out.len(), n + 1);
                return;
            }
        }
    }
}
