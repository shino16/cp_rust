---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::graph::*;\nuse crate::int::*;\nuse crate::ord::*;\n\npub fn\
    \ dijkstra<I: UInt, G: WGraph<I>>(g: &G, s: usize) -> Vec<I> {\n\tlet mut dist\
    \ = vec![I::MAX; g.len()];\n\tdist[s] = I::ZERO;\n\tlet mut togo = vec![s];\n\t\
    while let Some(v) = togo.pop() {\n\t\tg.adj_w(v, |w, &e| {\n\t\t\tlet d = dist[v]\
    \ + e;\n\t\t\tif dist[w].chmin(d) {\n\t\t\t\ttogo.push(w);\n\t\t\t}\n\t\t});\n\
    \t}\n\tdist\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/dijkstra.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dijkstra.rs
layout: document
redirect_from:
- /library/src/graph/dijkstra.rs
- /library/src/graph/dijkstra.rs.html
title: src/graph/dijkstra.rs
---
