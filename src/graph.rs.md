---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/bf.rs
    title: src/graph/bf.rs
  - icon: ':warning:'
    path: src/graph/bfs.rs
    title: src/graph/bfs.rs
  - icon: ':warning:'
    path: src/graph/dfs.rs
    title: src/graph/dfs.rs
  - icon: ':warning:'
    path: src/graph/dfs/cpnts.rs
    title: src/graph/dfs/cpnts.rs
  - icon: ':warning:'
    path: src/graph/dfs/weighted.rs
    title: src/graph/dfs/weighted.rs
  - icon: ':warning:'
    path: src/graph/dfs_io.rs
    title: src/graph/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/dist.rs
    title: src/graph/dist.rs
  - icon: ':warning:'
    path: src/graph/grid.rs
    title: src/graph/grid.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs.rs
    title: src/graph/tree/dfs.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs/weighted.rs
    title: src/graph/tree/dfs/weighted.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/tree/dist.rs
    title: src/graph/tree/dist.rs
  - icon: ':warning:'
    path: src/graph/tree/euler_tour.rs
    title: src/graph/tree/euler_tour.rs
  - icon: ':warning:'
    path: src/graph/tsort.rs
    title: src/graph/tsort.rs
  - icon: ':warning:'
    path: src/graph/weighted.rs
    title: src/graph/weighted.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph.rs\n"
  code: "pub mod bf;\npub mod bfs;\npub mod dfs;\npub mod dfs_io;\npub mod dist;\n\
    pub mod grid;\npub mod io;\npub mod max_flow;\npub mod tree;\npub mod tsort;\n\
    pub mod weighted;\n\npub trait Graph {\n    fn len(&self) -> usize;\n    fn adj(&self,\
    \ v: usize, f: impl FnMut(usize));\n}\nimpl Graph for Vec<Vec<usize>> {\n    fn\
    \ len(&self) -> usize { self.len() }\n    fn adj(&self, v: usize, f: impl FnMut(usize))\
    \ {\n        self[v].iter().copied().for_each(f);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph.rs
  requiredBy:
  - src/graph/dfs.rs
  - src/graph/tree/dfs.rs
  - src/graph/tree/dfs/weighted.rs
  - src/graph/tree/dist.rs
  - src/graph/tree/dfs_io.rs
  - src/graph/tree/euler_tour.rs
  - src/graph/dfs/cpnts.rs
  - src/graph/dfs/weighted.rs
  - src/graph/tsort.rs
  - src/graph/weighted.rs
  - src/graph/dist.rs
  - src/graph/grid.rs
  - src/graph/dfs_io.rs
  - src/graph/bf.rs
  - src/graph/tree.rs
  - src/graph/bfs.rs
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/graph.rs
layout: document
redirect_from:
- /library/src/graph.rs
- /library/src/graph.rs.html
title: src/graph.rs
---
