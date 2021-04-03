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
    RuntimeError: bundler is not specified: src/graph/dfs/cpnts.rs\n"
  code: "pub use super::*;\n\n/// dfs-ord\npub fn components(g: &impl Graph) -> Vec<Vec<usize>>\
    \ {\n    let mut visited = new_bitset(g.len());\n    let mut groups = Vec::new();\n\
    \    for v in 0..g.len() {\n        if visited.set_bit(v) {\n            let mut\
    \ group = Vec::new();\n            dfs_impl(g, v, !0, &mut visited, &mut |v, _|\
    \ {\n                group.push(v);\n            });\n            groups.push(group);\n\
    \        }\n    }\n    groups\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  - src/graph/dfs.rs
  isVerificationFile: false
  path: src/graph/dfs/cpnts.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs/cpnts.rs
layout: document
redirect_from:
- /library/src/graph/dfs/cpnts.rs
- /library/src/graph/dfs/cpnts.rs.html
title: src/graph/dfs/cpnts.rs
---