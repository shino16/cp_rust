---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bound.rs
    title: src/bound.rs
  - icon: ':question:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/ford_fulkerson.rs
    title: src/graph/max_flow/ford_fulkerson.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
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
    RuntimeError: bundler is not specified: src/graph/max_flow/ford_fulkerson/edges.rs\n"
  code: "pub use super::*;\n\n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: Num\
    \ + Bound> {\n\tpub from: usize,\n\tpub to: usize,\n\tpub cap: C,\n\tpub flow:\
    \ C,\n}\n\nimpl<C: Num + Bound> FordFulkerson<C> {\n\tpub fn get_edge(&self, v:\
    \ usize, idx: usize) -> Edge<C> {\n\t\tlet e = self.graph[v][idx];\n\t\tlet rev\
    \ = self.graph[e.to][e.rev];\n\t\tEdge { from: v, to: e.to, cap: e.cap + rev.cap,\
    \ flow: rev.cap }\n\t}\n\tpub fn get_edges_from(&self, v: usize) -> Vec<Edge<C>>\
    \ {\n\t\t(0..self.graph[v].len()).map(|idx| self.get_edge(v, idx)).collect()\n\
    \t}\n}\n"
  dependsOn:
  - src/bound.rs
  - src/ds/bitset.rs
  - src/graph/max_flow/ford_fulkerson.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/ford_fulkerson/edges.rs
  requiredBy: []
  timestamp: '2021-01-30 12:54:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/max_flow/ford_fulkerson/edges.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/ford_fulkerson/edges.rs
- /library/src/graph/max_flow/ford_fulkerson/edges.rs.html
title: src/graph/max_flow/ford_fulkerson/edges.rs
---
