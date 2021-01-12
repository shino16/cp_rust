pub use crate::io::*;

impl IO {
	pub fn scan_tree(&mut self) -> (Vec<Vec<usize>>, usize) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let (Usize1(u), Usize1(v)) = self.scan();
			graph[u].push(v);
			graph[v].push(u);
		}
		(graph, n)
	}
	pub fn scan_graph(&mut self) -> (Vec<Vec<usize>>, usize, usize) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let (Usize1(u), Usize1(v)) = self.scan();
			graph[u].push(v);
			graph[v].push(u);
		}
		(graph, n, m)
	}
	pub fn scan_ditree(&mut self) -> (Vec<Vec<usize>>, usize) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let (Usize1(u), Usize1(v)) = self.scan();
			graph[u].push(v);
		}
		(graph, n)
	}
	pub fn scan_digraph(&mut self) -> (Vec<Vec<usize>>, usize, usize) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let (Usize1(u), Usize1(v)) = self.scan();
			graph[u].push(v);
		}
		(graph, n, m)
	}
	pub fn scan_wtree<W: Scan + Copy>(&mut self) -> (Vec<Vec<(usize, W)>>, usize) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let (Usize1(u), Usize1(v)) = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
			graph[v].push((u, w));
		}
		(graph, n)
	}
	pub fn scan_wgraph<W: Scan + Copy>(&mut self) -> (Vec<Vec<(usize, W)>>, usize, usize) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let (Usize1(u), Usize1(v)) = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
			graph[v].push((u, w));
		}
		(graph, n, m)
	}
	pub fn scan_wditree<W: Scan + Copy>(&mut self) -> (Vec<Vec<(usize, W)>>, usize) {
		let n = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..n - 1 {
			let (Usize1(u), Usize1(v)) = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
		}
		(graph, n)
	}
	pub fn scan_wdigraph<W: Scan + Copy>(&mut self) -> (Vec<Vec<(usize, W)>>, usize, usize) {
		let (n, m) = self.scan();
		let mut graph = vec![Vec::new(); n];
		for _ in 0..m {
			let (Usize1(u), Usize1(v)) = self.scan();
			let w: W = self.scan();
			graph[u].push((v, w));
		}
		(graph, n, m)
	}
}
