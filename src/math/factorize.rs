pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {
	let mut res = Vec::new();
	if n % 2 != 0 {
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
	res
}
