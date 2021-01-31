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
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
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
    \ Graph, A: Group>(g: &G, s: usize, alg: A) -> Vec<A::Item> {\n\tlet mut state\
    \ = vec![alg.unit(); g.len()];\n\tlet mut res = vec![alg.unit(); g.len()];\n\t\
    dfs_io(g, s, |v, par| {\n\t\tif let Out(v) = v {\n\t\t\tg.adj(v, |w| {\n\t\t\t\
    \tif w != par {\n\t\t\t\t\talg.op_to(state[w], &mut state[v]);\n\t\t\t\t}\n\t\t\
    \t});\n\t\t}\n\t});\n\tres[s] = state[s];\n\tdfs_io(g, s, |v, par| match v {\n\
    \t\tIn(v) =>\n\t\t\tif par != !0 {\n\t\t\t\talg.op_inv_to(state[v], &mut state[par]);\n\
    \t\t\t\talg.op_to(state[par], &mut state[v]);\n\t\t\t\tres[v] = state[v];\n\t\t\
    \t},\n\t\tOut(v) =>\n\t\t\tif par != !0 {\n\t\t\t\talg.op_inv_to(state[par], &mut\
    \ state[v]);\n\t\t\t\talg.op_to(state[v], &mut state[par]);\n\t\t\t},\n\t});\n\
    \tres\n}\n"
  dependsOn:
  - src/alg.rs
  - src/graph.rs
  - src/graph/tree.rs
  - src/graph/tree/dfs_io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree/reroot.rs
  requiredBy: []
  timestamp: '2021-01-31 21:43:13+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/reroot.rs
layout: document
redirect_from:
- /library/src/graph/tree/reroot.rs
- /library/src/graph/tree/reroot.rs.html
title: src/graph/tree/reroot.rs
---
