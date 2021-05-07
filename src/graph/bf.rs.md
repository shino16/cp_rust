---
data:
  _extendedDependsOn:
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/bf.rs\n"
  code: "pub use super::weighted::*;\npub use crate::bounded::*;\nuse crate::num::*;\n\
    \npub fn bellman_ford<G: WGraph>(g: &G, s: usize) -> Option<Vec<G::W>> where G::W:\
    \ INum + Bounded {\n    let mut d = vec![G::W::MAX; g.len()];\n    d[s] = G::W::ZERO;\n\
    \    for _ in 0..g.len() {\n        let mut done = false;\n        for v in 0..g.len()\
    \ {\n            if d[v] == G::W::MAX {\n                continue;\n         \
    \   }\n            g.adj_w(v, |to, w| if d[to] > d[v] + w {\n                d[to]\
    \ = d[v] + w;\n                done = true;\n            });\n        }\n    \
    \    if !done {\n            return Some(d);\n        }\n    }\n    None\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/graph.rs
  - src/graph/weighted.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/bf.rs
  requiredBy: []
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/bf.rs
layout: document
redirect_from:
- /library/src/graph/bf.rs
- /library/src/graph/bf.rs.html
title: src/graph/bf.rs
---
