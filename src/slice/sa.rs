pub fn sort_cyclic_shift<T: Copy + Default, F: FnMut(T) -> usize>(
    s: &[T],
    max: T,
    mut key: F,
) -> Vec<usize> {
    let max_key = key(max);
    let len = s.len();
    let (mut p, mut p2) = (vec![!0; len], vec![!0; len]);
    let (mut c, mut c2) = (vec![!0; len], vec![!0; len]);
    let mut cnt = vec![0; len.max(max_key)];
    for &s in s {
        cnt[key(s)] += 1;
    }
    cnt.iter_mut().fold(0, |s, e| {
        *e += s;
        *e
    });
    for i in (0..len).rev() {
        let c = &mut cnt[key(s[i])];
        *c -= 1;
        p[*c] = i;
    }
    let mut cls = 0;
    c[p[0]] = cls;
    for i in 0..len - 1 {
        if key(s[p[i]]) != key(s[p[i + 1]]) {
            cls += 1;
        }
        c[p[i + 1]] = cls;
    }
    cls += 1;
    let mut done = 1;
    while done < len {
        cnt[..cls].iter_mut().for_each(|e| *e = 0);
        for (&p, p2) in p.iter().zip(p2.iter_mut()) {
            *p2 = if p < done { p + len - done } else { p - done };
            cnt[c[*p2]] += 1;
        }
        cnt.iter_mut().fold(0, |s, e| {
            *e += s;
            *e
        });
        for &p2 in p2.iter().rev() {
            let c = &mut cnt[c[p2]];
            *c -= 1;
            p[*c] = p2;
        }
        let rev = |p| if p + done < len { p + done } else { p + done - len };
        cls = 0;
        c2[p[0]] = cls;
        for (&p, &pp) in p.iter().zip(p.iter().skip(1)) {
            if c[p] != c[pp] || c[rev(p)] != c[rev(pp)] {
                cls += 1;
            }
            c2[pp] = cls;
        }
        cls += 1;
        std::mem::swap(&mut c, &mut c2);
        done <<= 1;
    }
    p
}

pub fn suffix_array<T: Copy + Default, F: FnMut(T) -> usize>(
    s: &mut Vec<T>,
    zero: T,
    max: T,
    key: F,
) -> Vec<usize> {
    s.push(zero);
    let sa = sort_cyclic_shift(&s, max, key);
    s.pop();
    sa
}
