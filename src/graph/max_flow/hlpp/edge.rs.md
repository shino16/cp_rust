---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/hlpp.rs
    title: src/graph/max_flow/hlpp.rs
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
    RuntimeError: bundler is not specified: src/graph/max_flow/hlpp/edge.rs\n"
  code: "pub use super::*;\n\n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: Num\
    \ + Bounded> {\n    pub from: usize,\n    pub to: usize,\n    pub cap: C,\n  \
    \  pub flow: C,\n}\n\nimpl<C: Num + Bounded> Hlpp<C> {\n    pub fn get_edge(&self,\
    \ v: usize, idx: usize) -> Edge<C> {\n        let e = self.graph[v][idx];\n  \
    \      let rev = self.graph[e.to][e.rev];\n        Edge { from: v, to: e.to, cap:\
    \ e.cap + rev.cap, flow: rev.cap }\n    }\n    pub fn get_edges_from(&self, v:\
    \ usize) -> Vec<Edge<C>> {\n        (0..self.graph[v].len()).map(|idx| self.get_edge(v,\
    \ idx)).collect()\n    }\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/graph/max_flow/hlpp.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/hlpp/edge.rs
  requiredBy: []
  timestamp: '2021-02-13 20:22:55+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/max_flow/hlpp/edge.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/hlpp/edge.rs
- /library/src/graph/max_flow/hlpp/edge.rs.html
title: src/graph/max_flow/hlpp/edge.rs
---
