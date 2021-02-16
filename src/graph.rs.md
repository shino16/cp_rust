---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/bfs.rs
    title: src/graph/bfs.rs
  - icon: ':warning:'
    path: src/graph/dfs.rs
    title: src/graph/dfs.rs
  - icon: ':warning:'
    path: src/graph/dfs_io.rs
    title: src/graph/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  - icon: ':warning:'
    path: src/graph/grid.rs
    title: src/graph/grid.rs
  - icon: ':x:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':warning:'
    path: src/graph/tree/dfs.rs
    title: src/graph/tree/dfs.rs
  - icon: ':x:'
    path: src/graph/tree/dfs_io.rs
    title: src/graph/tree/dfs_io.rs
  - icon: ':warning:'
    path: src/graph/tree/reroot.rs
    title: src/graph/tree/reroot.rs
  - icon: ':warning:'
    path: src/graph/weighted.rs
    title: src/graph/weighted.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph.rs\n"
  code: "pub mod bfs;\npub mod dfs;\npub mod dfs_io;\npub mod dijkstra;\npub mod euler_tour;\n\
    pub mod grid;\npub mod io;\npub mod max_flow;\npub mod tree;\npub mod weighted;\n\
    \npub trait Graph {\n    fn len(&self) -> usize;\n    fn adj<F: FnMut(usize)>(&self,\
    \ v: usize, f: F);\n}\nimpl Graph for Vec<Vec<usize>> {\n    fn len(&self) ->\
    \ usize { self.len() }\n    fn adj<F: FnMut(usize)>(&self, v: usize, f: F) { self[v].iter().copied().for_each(f);\
    \ }\n}\nimpl<W> Graph for Vec<Vec<(usize, W)>> {\n    fn len(&self) -> usize {\
    \ self.len() }\n    fn adj<F: FnMut(usize)>(&self, v: usize, mut f: F) { self[v].iter().for_each(|&(v,\
    \ _)| f(v)) }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph.rs
  requiredBy:
  - src/graph/bfs.rs
  - src/graph/euler_tour.rs
  - src/graph/grid.rs
  - src/graph/tree.rs
  - src/graph/dijkstra.rs
  - src/graph/tree/reroot.rs
  - src/graph/tree/dfs.rs
  - src/graph/tree/dfs_io.rs
  - src/graph/weighted.rs
  - src/graph/dfs.rs
  - src/graph/dfs_io.rs
  timestamp: '2021-02-13 16:52:06+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/graph.rs
layout: document
redirect_from:
- /library/src/graph.rs
- /library/src/graph.rs.html
title: src/graph.rs
---
