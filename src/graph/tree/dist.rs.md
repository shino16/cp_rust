---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs/weighted.rs
    title: src/graph/tree/dfs/weighted.rs
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
    RuntimeError: bundler is not specified: src/graph/tree/dist.rs\n"
  code: "pub use super::dfs::weighted::*;\nuse crate::num::*;\n\npub fn dist<G: WGraph>(g:\
    \ &G, s: usize) -> Vec<G::W> where G::W: Num + Default {\n    let mut dist = vec![G::W::zero();\
    \ g.len()];\n    dfs(g, s, |v, p, w| if p != !0 {\n        dist[v] = dist[p] +\
    \ w;\n    });\n    dist\n}\n"
  dependsOn:
  - src/graph.rs
  - src/graph/tree/dfs/weighted.rs
  - src/graph/weighted.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree/dist.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/dist.rs
layout: document
redirect_from:
- /library/src/graph/tree/dist.rs
- /library/src/graph/tree/dist.rs.html
title: src/graph/tree/dist.rs
---
