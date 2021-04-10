---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/reroot.rs\n"
  code: '// pub use super::dfs_io::*;

    // pub use crate::alg::*;


    // pub fn rerooting_dp<T: Copy, M: Group<T>>(g: &impl Graph, s: usize, alg: M)
    -> Vec<T> {

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

    '
  dependsOn: []
  isVerificationFile: false
  path: src/graph/tree/reroot.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/reroot.rs
layout: document
redirect_from:
- /library/src/graph/tree/reroot.rs
- /library/src/graph/tree/reroot.rs.html
title: src/graph/tree/reroot.rs
---
