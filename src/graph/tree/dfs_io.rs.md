---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/tree.rs
    title: src/graph/tree.rs
  _extendedRequiredBy: []
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
    RuntimeError: bundler is not specified: src/graph/tree/dfs_io.rs\n"
  code: "pub use super::*;\n\n#[derive(Debug)]\npub enum InOut {\n    In(usize),\n\
    \    Out(usize),\n}\n\npub use InOut::*;\n\npub fn dfs_io<G: Graph, F: FnMut(InOut,\
    \ usize)>(g: &G, s: usize, mut f: F) {\n    let mut togo = vec![(s, !0)];\n  \
    \  while let Some((v, par)) = togo.pop() {\n        if  v > !v {\n           \
    \ f(Out(!v), par);\n        } else {\n            f(In(v), par);\n           \
    \ togo.push((!v, par));\n            g.adj(v, |w| {\n                if w != par\
    \ {\n                    togo.push((w, v));\n                }\n            });\n\
    \        }\n    }\n}\n"
  dependsOn:
  - src/graph.rs
  - src/graph/tree.rs
  isVerificationFile: false
  path: src/graph/tree/dfs_io.rs
  requiredBy: []
  timestamp: '2021-03-14 02:25:56+09:00'
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
