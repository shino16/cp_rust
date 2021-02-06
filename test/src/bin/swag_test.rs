// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_D&lang=ja

use lib::ds::swag::*;
use lib::io::*;

fn main() {
	let mut io = IO::new();
	let (n, Usize1(l)) = io.scan();
	let a = io.scan_vec::<u32>(n);
	let mut swag = Swag::new(MonoidImpl(|| !0, u32::min));
	swag.extend_from_slice(&a[..l]);
	io.iterln(
		a[l..].iter().map(|&a| {
			swag.push(a);
			let ans = swag.ask();
			swag.pop();
			ans
		}),
		" ",
	);
}
