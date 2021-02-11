---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
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
    RuntimeError: bundler is not specified: src/graph/tree/dfs.rs\n"
  code: "pub use super::*;\n\n/// f: (v, par)\npub fn dfs<G: Graph, F: FnMut(usize,\
    \ usize)>(g: &G, s: usize, mut f: F) {\n    let mut togo = vec![(s, !0)];\n  \
    \  while let Some((v, par)) = togo.pop() {\n        f(v, par);\n        g.adj(v,\
    \ |w| {\n            if w != par {\n                togo.push((w, v));\n     \
    \       }\n        });\n    }\n}\n\npub fn topological<G: Graph>(g: &G, s: usize)\
    \ -> Vec<(usize, usize)> {\n    let mut res = Vec::with_capacity(g.len());\n \
    \   dfs(g, s, |v, par| res.push((v, par)));\n    res\n}\n"
  dependsOn:
  - src/graph.rs
  - src/graph/tree.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree/dfs.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/dfs.rs
layout: document
redirect_from:
- /library/src/graph/tree/dfs.rs
- /library/src/graph/tree/dfs.rs.html
title: src/graph/tree/dfs.rs
---
