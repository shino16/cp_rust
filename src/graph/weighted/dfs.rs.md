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
    path: src/graph/weighted.rs
    title: src/graph/weighted.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/weighted/dfs.rs\n"
  code: "pub use super::*;\nuse crate::ds::bitset::*;\n\n/// f: (v, par, w)\npub fn\
    \ dfs<W: Copy + Default>(\n    g: &impl WGraph<W>,\n    s: usize,\n    mut f:\
    \ impl FnMut(usize, usize, W),\n) {\n    let mut visited = new_bitset(g.len());\n\
    \    visited.set_bit(s);\n    dfs_impl(g, s, !0, W::default(), &mut visited, &mut\
    \ f);\n}\n\nfn dfs_impl<W: Copy>(\n    g: &impl WGraph<W>,\n    v: usize,\n  \
    \  par: usize,\n    w: W,\n    visited: &mut [u32],\n    f: &mut impl FnMut(usize,\
    \ usize, W),\n) {\n    f(v, par, w);\n    g.adj_w(v, |to, w| {\n        if visited.set_bit(to)\
    \ {\n            dfs_impl(g, to, v, w, visited, f);\n        }\n    });\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  - src/graph.rs
  - src/graph/weighted.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/weighted/dfs.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/weighted/dfs.rs
layout: document
redirect_from:
- /library/src/graph/weighted/dfs.rs
- /library/src/graph/weighted/dfs.rs.html
title: src/graph/weighted/dfs.rs
---
