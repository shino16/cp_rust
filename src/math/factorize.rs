pub fn factorize(mut n: u32) -> Vec<(u32, u32)> {
	if n == 1 {
		return Vec::new();
	}
	if n < 4 {
		return vec![(n, 1)];
	}
	let mut res = Vec::new();
	if n % 2 == 0 {
		let t = n.trailing_zeros();
		res.push((2, t));
		n >>= t;
	}
	for d in (3..).step_by(2) {
		if d * d > n {
			break;
		}
		if n % d == 0 {
			let mut cnt = 1;
			n /= d;
			while n % d == 0 {
				cnt += 1;
				n /= d;
			}
			res.push((d, cnt));
		}
	}
	if n != 1 {
		res.push((n, 1));
	}
	res
}
