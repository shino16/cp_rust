---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/bf.rs
    title: src/graph/bf.rs
  - icon: ':warning:'
    path: src/graph/dfs/weighted.rs
    title: src/graph/dfs/weighted.rs
  - icon: ':warning:'
    path: src/graph/dist.rs
    title: src/graph/dist.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs/weighted.rs
    title: src/graph/tree/dfs/weighted.rs
  - icon: ':warning:'
    path: src/graph/tree/dist.rs
    title: src/graph/tree/dist.rs
  - icon: ':warning:'
    path: src/graph/tree/euler_tour.rs
    title: src/graph/tree/euler_tour.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/weighted.rs\n"
  code: "pub use super::*;\n\npub trait WGraph: Graph {\n    type W;\n    fn adj_w(&self,\
    \ v: usize, f: impl FnMut(usize, Self::W));\n}\nimpl WGraph for Vec<Vec<usize>>\
    \ {\n    type W = usize;\n    fn adj_w(&self, v: usize, mut f: impl FnMut(usize,\
    \ Self::W)) {\n        self[v].iter().for_each(|&v| f(v, 1));\n    }\n}\nimpl<W:\
    \ Copy> WGraph for Vec<Vec<(usize, W)>> {\n    type W = W;\n    fn adj_w(&self,\
    \ v: usize, mut f: impl FnMut(usize, W)) {\n        self[v].iter().for_each(|&(v,\
    \ w)| f(v, w));\n    }\n}\n"
  dependsOn:
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/weighted.rs
  requiredBy:
  - src/graph/tree/dfs/weighted.rs
  - src/graph/tree/dist.rs
  - src/graph/tree/euler_tour.rs
  - src/graph/dfs/weighted.rs
  - src/graph/dist.rs
  - src/graph/bf.rs
  timestamp: '2021-04-16 00:20:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/weighted.rs
layout: document
redirect_from:
- /library/src/graph/weighted.rs
- /library/src/graph/weighted.rs.html
title: src/graph/weighted.rs
---
