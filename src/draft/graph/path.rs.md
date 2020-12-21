---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::graph::ds::*;\nuse crate::int::*;\n\npub fn dijkstra<'g, G:\
    \ Graph<'g>>(g: &'g G, s: G::V) -> Dict<'g, G::V, G::E> where G::E: UInt {\n\t\
    let mut dist = g.make_dict(!G::E::ZERO);\n\tdist[s] = G::E::ZERO;\n\tlet mut togo\
    \ = vec![s];\n\twhile let Some(v) = togo.pop() {\n\t\tfor (w, e) in g.adj_ve(v)\
    \ {\n\t\t\tif dist[v] + e < dist[w] {\n\t\t\t\tdist[w] = dist[v] + e;\n\t\t\t\t\
    togo.push(w);\n\t\t\t}\n\t\t}\n\t}\n\tdist\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/graph/path.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/graph/path.rs
layout: document
redirect_from:
- /library/src/draft/graph/path.rs
- /library/src/draft/graph/path.rs.html
title: src/draft/graph/path.rs
---
