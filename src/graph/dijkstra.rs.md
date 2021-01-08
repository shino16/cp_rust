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
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/dijkstra.rs\n"
  code: "pub use super::*;\nuse crate::assign::*;\nuse crate::int::*;\n\npub fn dijkstra<I:\
    \ UInt, G: WGraph<I>>(g: &G, s: usize) -> Vec<I> {\n\tlet mut dist = vec![I::MAX;\
    \ g.len()];\n\tdist[s] = I::ZERO;\n\tlet mut togo = vec![s];\n\twhile let Some(v)\
    \ = togo.pop() {\n\t\tg.adj_w(v, |w, &e| {\n\t\t\tif assign_if(dist[v] + e, &mut\
    \ dist[w], |x, y| x < y) {\n\t\t\t\ttogo.push(w);\n\t\t\t}\n\t\t});\n\t}\n\tdist\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/dijkstra.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dijkstra.rs
layout: document
redirect_from:
- /library/src/graph/dijkstra.rs
- /library/src/graph/dijkstra.rs.html
title: src/graph/dijkstra.rs
---
