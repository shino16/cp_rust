---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':warning:'
    path: src/graph.rs
    title: src/graph.rs
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
    RuntimeError: bundler is not specified: src/graph/bfs.rs\n"
  code: "use std::collections::VecDeque;\n\npub use super::*;\nuse crate::ds::bitset::*;\n\
    \n/// f: (v, par)\npub fn bfs<G: Graph, F: FnMut(usize, usize)>(g: &G, s: usize,\
    \ mut f: F) {\n\tlet mut visited = new_bitset(g.len());\n\tlet mut togo: VecDeque<_>\
    \ = vec![(s, !0)].into();\n\tvisited.set_bit(s, true);\n\twhile let Some((v, par))\
    \ = togo.pop_front() {\n\t\tf(v, par);\n\t\tg.adj(v, |w| {\n\t\t\tif visited.modify_bit(w,\
    \ true) {\n\t\t\t\ttogo.push_back((w, v));\n\t\t\t}\n\t\t})\n\t}\n}"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/bfs.rs
  requiredBy: []
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/bfs.rs
layout: document
redirect_from:
- /library/src/graph/bfs.rs
- /library/src/graph/bfs.rs.html
title: src/graph/bfs.rs
---
