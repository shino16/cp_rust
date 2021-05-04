---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/dfs/cpnts.rs
    title: src/graph/dfs/cpnts.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/dfs.rs\n"
  code: "pub use super::*;\nuse crate::ds::bitset::*;\n\npub mod cpnts;\npub mod weighted;\n\
    \npub fn dfs(g: &impl Graph, s: usize, mut f: impl FnMut(usize, usize)) {\n  \
    \  let mut visited = new_bitset(g.len());\n    visited.set_bit(s);\n    dfs_impl(g,\
    \ s, !0, &mut visited, &mut f);\n}\n\npub fn dfs_ord_par(g: &impl Graph, s: usize)\
    \ -> (Vec<usize>, Vec<usize>) {\n    let mut ord = Vec::with_capacity(g.len());\n\
    \    let mut par = vec![!0; g.len()];\n    dfs(g, s, |v, p| {\n        ord.push(v);\n\
    \        par[v] = p;\n    });\n    (ord, par)\n}\n\npub fn dfs_all(g: &impl Graph,\
    \ mut f: impl FnMut(usize, usize)) {\n    let mut visited = new_bitset(g.len());\n\
    \    for s in 0..g.len() {\n        if visited.set_bit(s) {\n            dfs_impl(g,\
    \ s, !0, &mut visited, &mut f);\n        }\n    }\n}\n\nfn dfs_impl(\n    g: &impl\
    \ Graph,\n    v: usize,\n    par: usize,\n    visited: &mut [u32],\n    f: &mut\
    \ impl FnMut(usize, usize),\n) {\n    f(v, par);\n    g.adj(v, |w| {\n       \
    \ if visited.set_bit(w) {\n            dfs_impl(g, w, v, visited, f);\n      \
    \  }\n    });\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/dfs.rs
  requiredBy:
  - src/graph/dfs/cpnts.rs
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs.rs
layout: document
redirect_from:
- /library/src/graph/dfs.rs
- /library/src/graph/dfs.rs.html
title: src/graph/dfs.rs
---
