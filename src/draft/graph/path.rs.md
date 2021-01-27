---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':x:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':warning:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':x:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/graph/path.rs\n"
  code: "pub use crate::graph::ds::*;\nuse crate::int::*;\n\npub fn dijkstra<'g, G:\
    \ Graph<'g>>(g: &'g G, s: G::V) -> Dict<'g, G::V, G::E> where G::E: UInt {\n\t\
    let mut dist = g.make_dict(!G::E::ZERO);\n\tdist[s] = G::E::ZERO;\n\tlet mut togo\
    \ = vec![s];\n\twhile let Some(v) = togo.pop() {\n\t\tfor (w, e) in g.adj_ve(v)\
    \ {\n\t\t\tif dist[v] + e < dist[w] {\n\t\t\t\tdist[w] = dist[v] + e;\n\t\t\t\t\
    togo.push(w);\n\t\t\t}\n\t\t}\n\t}\n\tdist\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/graph.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/draft/graph/path.rs
  requiredBy: []
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/graph/path.rs
layout: document
redirect_from:
- /library/src/draft/graph/path.rs
- /library/src/draft/graph/path.rs.html
title: src/draft/graph/path.rs
---
