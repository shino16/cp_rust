---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/ds/sparse_table.rs
    title: src/ds/sparse_table.rs
  - icon: ':warning:'
    path: src/ds/uslice.rs
    title: src/ds/uslice.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/uvec.rs
    title: src/ds/uvec.rs
  - icon: ':warning:'
    path: src/ds/uvec2d.rs
    title: src/ds/uvec2d.rs
  - icon: ':warning:'
    path: src/float/conv.rs
    title: src/float/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/gf/conv.rs
    title: src/gf/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds.rs\n"
  code: 'pub mod bitset;

    pub mod dst;

    pub mod fwk;

    pub mod idx;

    pub mod list;

    pub mod pool;

    pub mod segtree;

    pub mod sparse_table;

    pub mod swag;

    pub mod uf;

    pub mod uslice;

    pub mod uvec;

    pub mod uvec2d;

    pub mod vec2d;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/ds.rs
  requiredBy:
  - src/float/conv.rs
  - src/u64/conv.rs
  - src/ds/uvec.rs
  - src/ds/uslice.rs
  - src/ds/uvec2d.rs
  - src/ds/sparse_table.rs
  - src/mint/conv.rs
  - src/gf/conv.rs
  timestamp: '2021-04-11 12:36:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_test.rs
documentation_of: src/ds.rs
layout: document
redirect_from:
- /library/src/ds.rs
- /library/src/ds.rs.html
title: src/ds.rs
---
