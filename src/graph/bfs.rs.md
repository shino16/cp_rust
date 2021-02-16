---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':x:'
    path: src/graph.rs
    title: src/graph.rs
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
    RuntimeError: bundler is not specified: src/graph/bfs.rs\n"
  code: "use std::collections::VecDeque;\n\npub use super::*;\nuse crate::ds::bitset::*;\n\
    \n/// f: (v, par)\npub fn bfs<G: Graph, F: FnMut(usize, usize)>(g: &G, s: usize,\
    \ mut f: F) {\n    let mut visited = new_bitset(g.len());\n    let mut togo: VecDeque<_>\
    \ = vec![s].into();\n    visited.set_bit(s, true);\n    f(s, !0);\n    while let\
    \ Some(v) = togo.pop_front() {\n        g.adj(v, |w| {\n            if visited.modify_bit(w,\
    \ true) {\n                f(w, v);\n                togo.push_back(w);\n    \
    \        }\n        });\n    }\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/bfs.rs
  requiredBy: []
  timestamp: '2021-02-15 17:55:41+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/bfs.rs
layout: document
redirect_from:
- /library/src/graph/bfs.rs
- /library/src/graph/bfs.rs.html
title: src/graph/bfs.rs
---
