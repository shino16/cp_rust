// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A

use lib::ds::uf::*;
use lib::io::*;

fn main() {
	let mut io = IO::new();
	let (n, q) = io.scan();
	let mut uf = UnionFind::new(n);
	for _ in 0_usize..q {
		let (com, x, y): (u8, _, _) = io.scan();
		if com == b'0' {
			uf.unite(x, y);
		} else {
			io.println(uf.is_same(x, y) as u32);
		}
	}
}
