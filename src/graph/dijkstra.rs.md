---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/assign.rs
    title: src/assign.rs
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
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
    RuntimeError: bundler is not specified: src/graph/dijkstra.rs\n"
  code: "pub use super::*;\nuse crate::assign::*;\nuse crate::int::*;\nuse std::cmp::Reverse;\n\
    use std::collections::BinaryHeap;\n\npub fn dijkstra<I: UInt, G: WGraph<I>>(g:\
    \ &G, s: usize) -> Vec<I> {\n    let mut dist = vec![I::MAX; g.len()];\n    dist[s]\
    \ = I::ZERO;\n    let mut togo: BinaryHeap<_> = vec![Reverse((I::ZERO, s))].into();\n\
    \    while let Some(Reverse((d, v))) = togo.pop() {\n        g.adj_w(v, |w, &e|\
    \ {\n            if assign_if(d + e, &mut dist[w], |x, y| x < y) {\n         \
    \       togo.push(Reverse((d + e, w)));\n            }\n        });\n    }\n \
    \   dist\n}\n"
  dependsOn:
  - src/assign.rs
  - src/bit.rs
  - src/bounded.rs
  - src/cast.rs
  - src/graph.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/dijkstra.rs
  requiredBy: []
  timestamp: '2021-02-08 23:15:08+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dijkstra.rs
layout: document
redirect_from:
- /library/src/graph/dijkstra.rs
- /library/src/graph/dijkstra.rs.html
title: src/graph/dijkstra.rs
---