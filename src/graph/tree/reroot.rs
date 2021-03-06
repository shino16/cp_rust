// pub use super::dfs_io::*;
// pub use crate::alg::*;

// pub fn rerooting_dp<G: Graph, T: Copy, M: Group<T>>(g: &G, s: usize, alg: M) -> Vec<T> {
//     let mut state = vec![alg.unit(); g.len()];
//     let mut res = vec![alg.unit(); g.len()];
//     dfs_io(g, s, |v, par| {
//         if let Out(v) = v {
//             g.adj(v, |w| {
//                 if w != par {
//                     alg.op_to(state[w], &mut state[v]);
//                 }
//             });
//         }
//     });
//     res[s] = state[s];
//     dfs_io(g, s, |v, par| match v {
//         In(v) =>
//             if par != !0 {
//                 alg.op_inv_to(state[v], &mut state[par]);
//                 alg.op_to(state[par], &mut state[v]);
//                 res[v] = state[v];
//             },
//         Out(v) =>
//             if par != !0 {
//                 alg.op_inv_to(state[par], &mut state[v]);
//                 alg.op_to(state[v], &mut state[par]);
//             },
//     });
//     res
// }
