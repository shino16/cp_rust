pub use crate::alg::*;
use crate::ds::bitset::*;
pub use crate::graph::*;

pub fn euler_tour<A: Group, G: WTree<A::Item>>(g: &G, s: usize, a: A) -> Vec<A::Item>
where
	A::Item: Clone,
{
	let mut edges = Vec::new();
	let mut togo = vec![(s, !0, a.unit())];
	while let Some((v, par, e)) = togo.pop() {
		if v.get_bit(31) {
			edges.push(a.inv(e));
		} else {
			edges.push(e.clone());
			togo.push((!v, 0, e.clone()));
			g.adj_w(v, |w, e| {
				if w != par {
					togo.push((w, v, e.clone()));
				}
			})
		}
	}
	edges
}
