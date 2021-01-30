// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A

use lib::graph::max_flow::hlpp::Hlpp;
use lib::io::*;

fn main() {
	let mut io = IO::new();
	let [n, m]: [usize; 2] = io.scan();
	let mut solver = Hlpp::<u32>::new(n);
	for _ in 0..m {
		solver.add_edge(io.scan(), io.scan(), io.scan());
	}
	io.println(solver.solve(0, n - 1));
}
