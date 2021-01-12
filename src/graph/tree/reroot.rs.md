---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':warning:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':warning:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':warning:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/reroot.rs\n"
  code: "use std::cell::RefCell;\npub use super::dfs_io::*;\npub use crate::alg::*;\n\
    \npub fn rerooting<G: Graph, A: Group>(g: &G, s: usize, alg: A) -> Vec<A::Item>\
    \ {\n\tlet state = RefCell::new(vec![alg.unit(); g.len()]);\n\tlet mut res = vec![alg.unit();\
    \ g.len()];\n\tdfs_io(g, s, |_, _| {}, |v, par| {\n\t\tlet mut state = state.borrow_mut();\n\
    \t\tg.adj(v, |w| {\n\t\t\tif w != par {\n\t\t\t\talg.op_to(state[w], &mut state[v]);\n\
    \t\t\t}\n\t\t});\n\t});\n\tres[s] = state.borrow()[s];\n\tlet f_in = |v: usize,\
    \ par: usize| {\n\t\tlet mut state = state.borrow_mut();\n\t\tif par != !0 {\n\
    \t\t\talg.op_inv_to(state[v], &mut state[par]);\n\t\t\talg.op_to(state[par], &mut\
    \ state[v]);\n\t\t\tres[v] = state[v];\n\t\t}\n\t};\n\tlet f_out = |v: usize,\
    \ par: usize| {\n\t\tlet mut state = state.borrow_mut();\n\t\tif par != !0 {\n\
    \t\t\talg.op_inv_to(state[par], &mut state[v]);\n\t\t\talg.op_to(state[v], &mut\
    \ state[par]);\n\t\t}\n\t};\n\tdfs_io(g, s, f_in, f_out);\n\tres\n}\n"
  dependsOn:
  - src/alg.rs
  - src/ds/bitset.rs
  - src/graph.rs
  - src/graph/tree.rs
  - src/graph/tree/dfs_io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree/reroot.rs
  requiredBy: []
  timestamp: '2021-01-12 14:31:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/reroot.rs
layout: document
redirect_from:
- /library/src/graph/tree/reroot.rs
- /library/src/graph/tree/reroot.rs.html
title: src/graph/tree/reroot.rs
---
