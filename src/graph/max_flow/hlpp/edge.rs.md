---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bound.rs
    title: src/bound.rs
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
  code: "#[cfg(debug_assertions)]\nmacro_rules! dbg {\n    () => {\n        std::eprintln!(\"\
    [{}:{}]\", std::file!(), std::line!());\n    };\n    ($val:expr) => {\n      \
    \  // Use of `match` here is intentional because it affects the lifetimes\n  \
    \      // of temporaries - https://stackoverflow.com/a/48732525/1063961\n    \
    \    match $val {\n            tmp => {\n                std::eprintln!(\"[{}:{}]\
    \ {} = {:?}\",\n                    std::file!(), std::line!(), std::stringify!($val),\
    \ &tmp);\n                tmp\n            }\n        }\n    };\n    // Trailing\
    \ comma with single argument is ignored\n    ($val:expr,) => { dbg!($val) };\n\
    \    ($($val:expr),+ $(,)?) => {\n        ($(dbg!($val)),+,)\n    };\n}\n\n#[cfg(not(debug_assertions))]\n\
    macro_rules! dbg {\n\t($($x:expr),*) => { std::convert::identity(($($x),*)) }\n\
    }\n\npub use super::*;\n\n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: Num\
    \ + Bound> {\n\tpub from: usize,\n\tpub to: usize,\n\tpub cap: C,\n\tpub flow:\
    \ C,\n}\n\nimpl<C: Num + Bound> Hlpp<C> {\n\tpub fn get_edge(&self, v: usize,\
    \ idx: usize) -> Edge<C> {\n\t\tlet e = self.graph[v][idx];\n\t\tlet rev = self.graph[e.to][e.rev];\n\
    \t\tEdge { from: v, to: e.to, cap: e.cap + rev.cap, flow: rev.cap }\n\t}\n\tpub\
    \ fn get_edges_from(&self, v: usize) -> Vec<Edge<C>> {\n\t\t(0..self.graph[v].len()).map(|idx|\
    \ self.get_edge(v, idx)).collect()\n\t}\n\tpub fn dbg_edges(&self) {\n\t\tfor\
    \ v in 0..self.len() {\n\t\t\tdbg!(v, self.get_edges_from(v));\n\t\t}\n\t\tdbg!(self.excess[7]);\n\
    \t}\n}\n"
  dependsOn:
  - src/bound.rs
  - src/graph/max_flow/hlpp.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/hlpp/edge.rs
  requiredBy: []
  timestamp: '2021-01-30 12:54:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/max_flow/hlpp/edge.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/hlpp/edge.rs
- /library/src/graph/max_flow/hlpp/edge.rs.html
title: src/graph/max_flow/hlpp/edge.rs
---
