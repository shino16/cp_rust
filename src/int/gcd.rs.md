---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/int/gcd.rs\n"
  code: "use super::*;\n\npub fn gcd<I: Int>(a: I, b: I) -> I {\n    ugcd(a.abs(),\
    \ b.abs()).as_()\n}\n\n// binary gcd\npub fn ugcd<I: UInt>(a: I, b: I) -> I {\n\
    \    #[target_feature(enable = \"bmi1\")]\n    unsafe fn ugcd_impl<I: UInt>(mut\
    \ a: I, mut b: I) -> I {\n        if a.is_zero() {\n            return b;\n  \
    \      } else if b.is_zero() {\n            return a;\n        }\n        let\
    \ a_shift = a.trailing_zeros();\n        a >>= a_shift;\n        let b_shift =\
    \ b.trailing_zeros();\n        b >>= b_shift;\n        while a != b {\n      \
    \      if a > b {\n                std::mem::swap(&mut a, &mut b);\n         \
    \   }\n            b -= a;\n            b >>= b.trailing_zeros();\n        }\n\
    \        a << a_shift.min(b_shift)\n    }\n    unsafe {\n        ugcd_impl(a,\
    \ b)\n    }\n}\n\n/// (x, y, g) where ax + by = g, x >= 0\npub fn extgcd<I: IInt>(mut\
    \ a: I, mut b: I) -> (I, I, I) {\n    // A = [a, x, y; b, u, v], k = [-1; a0;\
    \ b0]\n    // A'= [a, x, y; 0, u, v] \\therefore a0*u + b0*v = 0\n    let (mut\
    \ x, mut y, mut u, mut v) = (I::ONE, I::ZERO, I::ZERO, I::ONE);\n    while !b.is_zero()\
    \ {\n        let t = a / b;\n        a -= t * b;\n        x -= t * u;\n      \
    \  y -= t * v;\n        std::mem::swap(&mut a, &mut b);\n        std::mem::swap(&mut\
    \ x, &mut u);\n        std::mem::swap(&mut y, &mut v);\n    }\n    if x < I::ZERO\
    \ {\n        x += u;\n        y -= v;\n        debug_assert_eq!(gcd(u, v), I::ONE);\n\
    \        debug_assert!(x + u >= I::ZERO);\n    }\n    (x, y, a)\n}\n"
  dependsOn:
  - src/bit.rs
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/gcd.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/segtree_beats_test.rs
documentation_of: src/int/gcd.rs
layout: document
redirect_from:
- /library/src/int/gcd.rs
- /library/src/int/gcd.rs.html
title: src/int/gcd.rs
---
