use super::sort::*;

// reference
// Kärkkäinen, Juha, Peter Sanders, and Stefan Burkhardt. "Linear work suffix array construction." Journal of the ACM (JACM) 53, no. 6, pp. 918-936, 2006.
pub fn suffix_array<T, F: FnMut(&T) -> usize>(
	t: &[T],
	out: &mut Vec<usize>,
	max_key: usize,
	mut key: F,
) {
	out.clear();
	out.reserve(t.len() + 1);
	out.push(t.len());

	if t.len() == 0 {
		return;
	} else if t.len() == 1 {
		out.push(1);
		return;
	}

	let n = t.len();
	let mut key_at = |i| if i >= n { 0 } else { key(&t[i]) };
	let (n0, n1, n2) = ((n + 2) / 3, (n + 1) / 3, n / 3);
	let n02 = n0 + n2;
	let (mut r, mut sa12) = (Vec::with_capacity(n02 + 3), Vec::with_capacity(n02));
	for i in 0..n + (n0 - n1) {
		if i % 3 != 0 {
			r.push(i);
		}
	}
	assert_eq!(r.len(), n02);

	// radix sort [(t[i], t[i + 1], t[i + 2]) | i % 3 != 0]
	count_sort(&r, &mut sa12, max_key, |&v| key_at(v + 2));
	count_sort(&sa12, &mut r, max_key, |&v| key_at(v + 1));
	count_sort(&r, &mut sa12, max_key, |&v| key_at(v));

	let (mut name, mut c0, mut c1, mut c2) = (0, !0, !0, !0);
	for &i in &sa12 {
		if key_at(i) != c0 || key_at(i + 1) != c1 || key_at(i + 2) != c2 {
			name += 1;
			c0 = key_at(i);
			c1 = key_at(i + 1);
			c2 = key_at(i + 2);
		}
		if i % 3 == 1 {
			r[i / 3] = name;
		} else {
			r[i / 3 + n0] = name;
		}
	}
	if name < n02 {
		fn deref(v: &usize) -> usize {
			*v
		}
		suffix_array(&r, &mut sa12, name, deref);
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
	count_sort(&r0, &mut sa0, max_key, |&v| key_at(v));

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
			(key_at(i), r[qi + n0]) <= (key_at(j), r[j / 3])
		} else {
			(key_at(i), key_at(i + 1), r[qi - n0 + 1])
				<= (key_at(j), key_at(j + 1), r[j / 3 + n0])
		};
		if cond {
			out.push(i);
			if let Some(q) = q.next() {
				qi = q;
			} else {
				out.push(j);
				out.extend(p);
				return;
			}
		} else {
			out.push(j);
			if let Some(p) = p.next() {
				pi = p;
			} else {
				out.push(i);
				out.extend(q.map(enc));
				return;
			}
		}
	}
}
