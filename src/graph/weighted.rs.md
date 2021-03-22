---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/weighted.rs\n"
  code: "pub use super::*;\nuse crate::zo::ZeroOne;\n\npub trait WGraph<W: Copy>:\
    \ Graph {\n    fn adj_w<F: FnMut(usize, W)>(&self, v: usize, f: F);\n}\nimpl<T:\
    \ ZeroOne> WGraph<T> for Vec<Vec<usize>> {\n    fn adj_w<F: FnMut(usize, T)>(&self,\
    \ v: usize, mut f: F) {\n        self[v].iter().for_each(|&v| f(v, T::ONE));\n\
    \    }\n}\nimpl<W: Copy> WGraph<W> for Vec<Vec<(usize, W)>> {\n    fn adj_w<F:\
    \ FnMut(usize, W)>(&self, v: usize, mut f: F) {\n        self[v].iter().for_each(|&(v,\
    \ w)| f(v, w));\n    }\n}\n"
  dependsOn:
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/weighted.rs
  requiredBy:
  - src/graph/dijkstra.rs
  - src/graph/euler_tour.rs
  timestamp: '2021-03-22 00:48:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/weighted.rs
layout: document
redirect_from:
- /library/src/graph/weighted.rs
- /library/src/graph/weighted.rs.html
title: src/graph/weighted.rs
---
