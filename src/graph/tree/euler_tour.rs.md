---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
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
    RuntimeError: bundler is not specified: src/graph/tree/euler_tour.rs\n"
  code: "pub use crate::alg::arith::*;\npub use crate::graph::weighted::*;\n\npub\
    \ fn euler_tour<G: WGraph>(\n    g: &G,\n    s: usize,\n    alg: impl Group<G::W>,\n\
    ) -> (Vec<G::W>, Vec<usize>, Vec<usize>, Vec<(usize, usize)>)\nwhere\n    G::W:\
    \ Copy,\n{\n    let mut edges = Vec::with_capacity(g.len() * 2);\n    let mut\
    \ togo = Vec::with_capacity(g.len() * 2);\n    g.adj_w(s, |to, e| togo.push((to,\
    \ s, e)));\n    let mut range = vec![(!0, !0); g.len()];\n    let mut us = Vec::with_capacity(g.len()\
    \ * 2);\n    let mut vs = us.clone();\n    while let Some((v, par, e)) = togo.pop()\
    \ {\n        if v > !v {\n            range[!v].1 = edges.len();\n           \
    \ edges.push(alg.inv(e));\n            us.push(!v);\n            vs.push(par);\n\
    \        } else {\n            edges.push(e);\n            us.push(par);\n   \
    \         vs.push(v);\n            range[v].0 = edges.len();\n            togo.push((!v,\
    \ par, e));\n            g.adj_w(v, |w, e| {\n                if w != par {\n\
    \                    togo.push((w, v, e));\n                }\n            });\n\
    \        }\n    }\n    range[s] = (0, edges.len());\n    (edges, us, vs, range)\n\
    }\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/graph.rs
  - src/graph/weighted.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree/euler_tour.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/euler_tour.rs
layout: document
redirect_from:
- /library/src/graph/tree/euler_tour.rs
- /library/src/graph/tree/euler_tour.rs.html
title: src/graph/tree/euler_tour.rs
---