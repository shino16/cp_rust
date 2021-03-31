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
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/dfs.rs\n"
  code: "pub use super::*;\n\n/// f: (v, par)\npub fn dfs(g: &impl Graph, s: usize,\
    \ mut f: impl FnMut(usize, usize)) {\n    let mut togo = vec![(s, !0)];\n    while\
    \ let Some((v, par)) = togo.pop() {\n        f(v, par);\n        g.adj(v, |w|\
    \ {\n            if w != par {\n                togo.push((w, v));\n         \
    \   }\n        });\n    }\n}\n\npub fn dfs_ord_par(g: &impl Graph, s: usize) ->\
    \ (Vec<usize>, Vec<usize>) {\n    let mut ord = Vec::with_capacity(g.len());\n\
    \    let mut par = vec![!0; g.len()];\n    dfs(g, s, |v, p| {\n        ord.push(v);\n\
    \        par[v] = p;\n    });\n    (ord, par)\n}\n"
  dependsOn:
  - src/graph.rs
  - src/graph/tree.rs
  isVerificationFile: false
  path: src/graph/tree/dfs.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/dfs.rs
layout: document
redirect_from:
- /library/src/graph/tree/dfs.rs
- /library/src/graph/tree/dfs.rs.html
title: src/graph/tree/dfs.rs
---
