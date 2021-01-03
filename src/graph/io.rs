pub use crate::io::*;

impl IO {
	pub fn scan_tree(&mut self) -> (usize, Vec<Vec<usize>>) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			graph[u].push(v);
			graph[v].push(u);
		}
		(n, graph)
	}
	pub fn scan_graph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			graph[u].push(v);
			graph[v].push(u);
		}
		(n, m, graph)
	}
	pub fn scan_ditree(&mut self) -> (usize, Vec<Vec<usize>>) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			graph[u].push(v);
		}
		(n, graph)
	}
	pub fn scan_digraph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			graph[u].push(v);
		}
		(n, m, graph)
	}
	pub fn scan_wtree<W: Scan + Copy>(&mut self) -> (usize, Vec<Vec<(usize, W)>>) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
			graph[v].push((u, w));
		}
		(n, graph)
	}
	pub fn scan_wgraph<W: Scan + Copy>(&mut self) -> (usize, usize, Vec<Vec<(usize, W)>>) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
			graph[v].push((u, w));
		}
		(n, m, graph)
	}
	pub fn scan_wditree<W: Scan + Copy>(&mut self) -> (usize, Vec<Vec<(usize, W)>>) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
		}
		(n, graph)
	}
	pub fn scan_wdigraph<W: Scan + Copy>(&mut self) -> (usize, usize, Vec<Vec<(usize, W)>>) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let [Usize1(u), Usize1(v)]: [Usize1; 2] = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
		}
		(n, m, graph)
	}
}
