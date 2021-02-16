---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/reroot.rs\n"
  code: "pub use super::dfs_io::*;\npub use crate::alg::*;\n\npub fn rerooting_dp<G:\
    \ Graph, T: Copy, M: Group<T>>(g: &G, s: usize, alg: M) -> Vec<T> {\n    let mut\
    \ state = vec![alg.unit(); g.len()];\n    let mut res = vec![alg.unit(); g.len()];\n\
    \    dfs_io(g, s, |v, par| {\n        if let Out(v) = v {\n            g.adj(v,\
    \ |w| {\n                if w != par {\n                    alg.op_to(state[w],\
    \ &mut state[v]);\n                }\n            });\n        }\n    });\n  \
    \  res[s] = state[s];\n    dfs_io(g, s, |v, par| match v {\n        In(v) =>\n\
    \            if par != !0 {\n                alg.op_inv_to(state[v], &mut state[par]);\n\
    \                alg.op_to(state[par], &mut state[v]);\n                res[v]\
    \ = state[v];\n            },\n        Out(v) =>\n            if par != !0 {\n\
    \                alg.op_inv_to(state[par], &mut state[v]);\n                alg.op_to(state[v],\
    \ &mut state[par]);\n            },\n    });\n    res\n}\n"
  dependsOn:
  - src/alg.rs
  - src/graph.rs
  - src/graph/tree.rs
  - src/graph/tree/dfs_io.rs
  isVerificationFile: false
  path: src/graph/tree/reroot.rs
  requiredBy: []
  timestamp: '2021-02-17 07:58:47+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/reroot.rs
layout: document
redirect_from:
- /library/src/graph/tree/reroot.rs
- /library/src/graph/tree/reroot.rs.html
title: src/graph/tree/reroot.rs
---
