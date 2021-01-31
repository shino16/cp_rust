// verify-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use lib::alg::arith::*;
use lib::ds::segtree::lazy::*;
use lib::io::*;
use lib::mint::*;

fn main() {
	let mut io = IO::new();
	let [n, q]: [usize; 2] = io.scan();
	let a = io.scan_iter::<Mint99>(n).map(|a| (a, Mint99::ONE)).collect::<Vec<_>>();
	let mut ds = LazySegmentTree::new_from_slice(
		&a,
		MonoidImpl(|| (Mint99::ZERO, Mint99::ZERO), |(a, s), (b, t)| (a + b, s + t)),
		MonoidImpl(|| (Mint99::ONE, Mint99::ZERO), |(a, b), (c, d)| (a * c, b * c + d)),
		|(x, w), (a, b)| (a * x + b * w, w),
	);
	for _ in 0..q {
		let t: u32 = io.scan();
		if t == 0 {
			ds.act_over(io.scan(), io.scan(), io.scan());
		} else {
			let ans = ds.ask(io.scan(), io.scan()).0;
			io.println(ans);
		}
	}
}

// f(x) = ax + b
// g(x) = cx + d
// g(f(x)) = acx + bc + d
