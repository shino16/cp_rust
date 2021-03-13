---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/poly.rs
    title: src/poly.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/conv.rs\n"
  code: "pub trait Conv: Sized {\n    fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>)\
    \ -> Vec<Self> {\n        Conv::conv_in_place(&mut lhs, &mut rhs);\n        lhs\n\
    \    }\n    fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>);\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/conv.rs
  requiredBy:
  - src/u64/conv.rs
  - src/fp/conv.rs
  - src/poly.rs
  - src/mint/conv.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/conv.rs
layout: document
redirect_from:
- /library/src/conv.rs
- /library/src/conv.rs.html
title: src/conv.rs
---
