---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/assign.rs
    title: src/assign.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':warning:'
    path: src/graph/weighted.rs
    title: src/graph/weighted.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/dist.rs\n"
  code: "pub use super::weighted::*;\nuse crate::assign::*;\nuse crate::bounded::*;\n\
    use crate::num::*;\nuse std::cmp::Reverse;\nuse std::collections::BinaryHeap;\n\
    \npub fn dijkstra<G: WGraph>(g: &G, s: usize) -> Vec<G::W> where G::W: Num + Bounded\
    \ {\n    let mut dist = vec![G::W::MAX; g.len()];\n    dist[s] = G::W::ZERO;\n\
    \    let mut togo: BinaryHeap<_> = vec![Reverse((G::W::ZERO, s))].into();\n  \
    \  while let Some(Reverse((d, v))) = togo.pop() {\n        g.adj_w(v, |w, e| {\n\
    \            if assign_if(d + e, &mut dist[w], |x, y| x < y) {\n             \
    \   togo.push(Reverse((d + e, w)));\n            }\n        });\n    }\n    dist\n\
    }\n"
  dependsOn:
  - src/assign.rs
  - src/bounded.rs
  - src/graph.rs
  - src/graph/weighted.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/dist.rs
  requiredBy: []
  timestamp: '2021-04-16 00:20:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dist.rs
layout: document
redirect_from:
- /library/src/graph/dist.rs
- /library/src/graph/dist.rs.html
title: src/graph/dist.rs
---
