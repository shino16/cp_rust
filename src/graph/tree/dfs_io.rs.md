---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/tree/reroot.rs
    title: src/graph/tree/reroot.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/dfs_io.rs\n"
  code: "pub use super::*;\n\n#[derive(Debug)]\npub enum InOut {\n\tIn(usize),\n\t\
    Out(usize),\n}\n\npub use InOut::*;\n\npub fn dfs_io<G: Graph, F: FnMut(InOut,\
    \ usize)>(g: &G, s: usize, mut f: F) {\n\tlet mut togo = vec![(s, !0)];\n\twhile\
    \ let Some((v, par)) = togo.pop() {\n\t\tif  v > !v {\n\t\t\tf(Out(!v), par);\n\
    \t\t} else {\n\t\t\tf(In(v), par);\n\t\t\ttogo.push((!v, par));\n\t\t\tg.adj(v,\
    \ |w| {\n\t\t\t\tif w != par {\n\t\t\t\t\ttogo.push((w, v));\n\t\t\t\t}\n\t\t\t\
    });\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/graph.rs
  - src/graph/tree.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/tree/dfs_io.rs
  requiredBy:
  - src/graph/tree/reroot.rs
  timestamp: '2021-01-31 21:43:13+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/graph/tree/dfs_io.rs
layout: document
redirect_from:
- /library/src/graph/tree/dfs_io.rs
- /library/src/graph/tree/dfs_io.rs.html
title: src/graph/tree/dfs_io.rs
---
