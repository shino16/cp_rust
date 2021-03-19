---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':warning:'
    path: src/graph/dfs.rs
    title: src/graph/dfs.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/dfs/cmpnt.rs\n"
  code: "pub use super::*;\n\npub fn components<G: Graph>(g: &G) -> Vec<Vec<usize>>\
    \ {\n    let mut visited = new_bitset(g.len());\n    let mut groups = Vec::new();\n\
    \    for v in 0..g.len() {\n        let mut group = Vec::new();\n        visited.set_bit(v);\n\
    \        _dfs_impl(g, v, !0, &mut visited, &mut |v, _| {\n            group.push(v);\n\
    \        });\n        groups.push(group);\n    }\n    groups\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  - src/graph/dfs.rs
  isVerificationFile: false
  path: src/graph/dfs/cmpnt.rs
  requiredBy: []
  timestamp: '2021-03-19 19:54:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs/cmpnt.rs
layout: document
redirect_from:
- /library/src/graph/dfs/cmpnt.rs
- /library/src/graph/dfs/cmpnt.rs.html
title: src/graph/dfs/cmpnt.rs
---
