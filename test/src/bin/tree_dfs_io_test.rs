// verification-helper: PROBLEM https://yukicoder.me/problems/no/778

use lib::ds::fenwick::*;
use lib::graph::tree::dfs_io::*;
use lib::io::*;

fn main() {
	let mut io = IO::new();
	let n = io.scan();
	let mut graph = vec![Vec::new(); n];
	for v in 1..n {
		let p: usize = io.scan();
		graph[p].push(v);
		graph[v].push(p);
	}
	let mut fwk = FenwickTree::new(vec![0; n], Addition::new());
	let mut ans = 0;
	dfs_io(&graph, 0, |v, _| match v {
		In(v) => {
			ans += fwk.ask_prefix(v) as u64;
			fwk.add(v, 1_u32);
		},
		Out(v) => fwk.sub(v, 1),
	});
	io.println(ans);
}
