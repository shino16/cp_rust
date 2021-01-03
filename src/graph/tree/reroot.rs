use std::cell::RefCell;
pub use super::dfs_io::*;
pub use crate::alg::*;

pub fn rerooting<G: Graph, A: Group>(g: &G, s: usize, alg: A) -> Vec<A::Item> {
	let state = RefCell::new(vec![alg.unit(); g.len()]);
	let mut res = vec![alg.unit(); g.len()];
	dfs_io(g, s, |_, _| {}, |v, par| {
		let mut state = state.borrow_mut();
		g.adj(v, |w| {
			if w != par {
				alg.op_to(state[w], &mut state[v]);
			}
		});
	});
	res[s] = state.borrow()[s];
	let f_in = |v: usize, par: usize| {
		let mut state = state.borrow_mut();
		if par != !0 {
			alg.op_inv_to(state[v], &mut state[par]);
			alg.op_to(state[par], &mut state[v]);
			res[v] = state[v];
		}
	};
	let f_out = |v: usize, par: usize| {
		let mut state = state.borrow_mut();
		if par != !0 {
			alg.op_inv_to(state[par], &mut state[v]);
			alg.op_to(state[v], &mut state[par]);
		}
	};
	dfs_io(g, s, f_in, f_out);
	res
}
