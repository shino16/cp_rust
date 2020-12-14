pub use crate::graph::ds::*;
use crate::int::*;

pub fn dijkstra<'g, G: Graph<'g>>(g: &'g G, s: G::V) -> Dict<'g, G::V, G::E> where G::E: UInt {
	let mut dist = g.make_dict(!G::E::ZERO);
	dist[s] = G::E::ZERO;
	let mut togo = vec![s];
	while let Some(v) = togo.pop() {
		for (w, e) in g.adj_ve(v) {
			if dist[v] + e < dist[w] {
				dist[w] = dist[v] + e;
				togo.push(w);
			}
		}
	}
	dist
}
