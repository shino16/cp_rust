---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/tree/dfs.rs
    title: src/graph/tree/dfs.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/tree/reroot.rs
    title: src/graph/tree/reroot.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree.rs\n"
  code: 'pub mod dfs;

    pub mod dfs_io;

    pub mod reroot;

    pub use super::*;

    '
  dependsOn:
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree.rs
  requiredBy:
  - src/graph/tree/dfs.rs
  - src/graph/tree/dfs_io.rs
  - src/graph/tree/reroot.rs
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree.rs
layout: document
redirect_from:
- /library/src/graph/tree.rs
- /library/src/graph/tree.rs.html
title: src/graph/tree.rs
---
