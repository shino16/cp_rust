---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
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
    RuntimeError: bundler is not specified: src/graph/dfs_io.rs\n"
  code: "pub use super::*;\nuse crate::ds::bitset::*;\n\n#[derive(Debug)]\npub enum\
    \ InOut {\n    In(usize),\n    Out(usize),\n}\n\npub use InOut::*;\n\npub fn dfs_io<G:\
    \ Graph, F: FnMut(InOut, usize)>(g: &G, s: usize, mut f: F) {\n    let mut visited\
    \ = new_bitset(g.len());\n    visited.set_bit(s, true);\n    let mut togo = vec![(s,\
    \ !0)];\n    while let Some((v, par)) = togo.pop() {\n        if v >> 31 != 0\
    \ {\n            f(Out(!v), par);\n        } else {\n            f(In(v), par);\n\
    \            togo.push((!v, par));\n            g.adj(v, |w| {\n             \
    \   if visited.modify_bit(w, true) {\n                    togo.push((w, v));\n\
    \                }\n            });\n        }\n    }\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/dfs_io.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs_io.rs
layout: document
redirect_from:
- /library/src/graph/dfs_io.rs
- /library/src/graph/dfs_io.rs.html
title: src/graph/dfs_io.rs
---
