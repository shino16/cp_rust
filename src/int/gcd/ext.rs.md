---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
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
    RuntimeError: bundler is not specified: src/int/gcd/ext.rs\n"
  code: "pub use crate::int::*;\n\n/// (x, y, g) where ax + by = g, x >= 0\npub fn\
    \ extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I, I) {\n    // A = [a, x, y; b,\
    \ u, v], k = [-1; a0; b0]\n    // A'= [a, x, y; 0, u, v] \\therefore a0*u + b0*v\
    \ = 0\n    let (mut x, mut y, mut u, mut v) = (I::ONE, I::ZERO, I::ZERO, I::ONE);\n\
    \    while !b.is_zero() {\n        let t = a / b;\n        a -= t * b;\n     \
    \   x -= t * u;\n        y -= t * v;\n        std::mem::swap(&mut a, &mut b);\n\
    \        std::mem::swap(&mut x, &mut u);\n        std::mem::swap(&mut y, &mut\
    \ v);\n    }\n    if x < I::ZERO {\n        x += u;\n        y -= v;\n       \
    \ // debug_assert_eq!(gcd(u, v), I::ONE);\n        debug_assert!(x + u >= I::ZERO);\n\
    \    }\n    (x, y, a)\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/gcd/ext.rs
  requiredBy: []
  timestamp: '2021-02-21 16:57:52+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/gcd/ext.rs
layout: document
redirect_from:
- /library/src/int/gcd/ext.rs
- /library/src/int/gcd/ext.rs.html
title: src/int/gcd/ext.rs
---
