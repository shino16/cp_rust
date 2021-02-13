---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
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
    RuntimeError: bundler is not specified: src/graph/dfs.rs\n"
  code: "pub use super::*;\nuse crate::ds::bitset::*;\n\n/// f: (v, par)\npub fn dfs<G:\
    \ Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {\n    let mut togo\
    \ = vec![(s, !0)];\n    let mut visited = new_bitset(g.len());\n    visited.set_bit(s,\
    \ true);\n    while let Some((v, par)) = togo.pop() {\n        f(v, par);\n  \
    \      g.adj(v, |w| {\n            if visited.modify_bit(w, true) {\n        \
    \        togo.push((w, v));\n            }\n        });\n    }\n}\n\npub fn is_connected<G:\
    \ Graph>(g: &G) -> bool {\n    let mut cnt = 0;\n    dfs(g, 0, |_, _| cnt += 1);\n\
    \    cnt == g.len()\n}\n\npub fn dfs_ord_par<G: Graph>(g: &G, s: usize) -> (Vec<(usize,\
    \ usize)>, Vec<usize>) {\n    let mut ord = Vec::with_capacity(g.len());\n   \
    \ let mut par = vec![!0; g.len()];\n    dfs(g, s, |v, p| {\n        ord.push((v,\
    \ p));\n        par[v] = p;\n    });\n    (ord, par)\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/dfs.rs
  requiredBy: []
  timestamp: '2021-02-13 16:52:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs.rs
layout: document
redirect_from:
- /library/src/graph/dfs.rs
- /library/src/graph/dfs.rs.html
title: src/graph/dfs.rs
---
