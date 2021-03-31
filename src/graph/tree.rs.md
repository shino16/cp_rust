---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/tree/dfs.rs
    title: src/graph/tree/dfs.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree.rs\n"
  code: 'pub mod dfs;

    pub mod dfs_io;

    pub mod pruefer;

    // pub mod reroot;

    pub use super::*;

    '
  dependsOn:
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/tree.rs
  requiredBy:
  - src/graph/tree/dfs.rs
  - src/graph/tree/dfs_io.rs
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/graph/tree.rs
layout: document
redirect_from:
- /library/src/graph/tree.rs
- /library/src/graph/tree.rs.html
title: src/graph/tree.rs
---
