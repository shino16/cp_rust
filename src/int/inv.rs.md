---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bits.rs
    title: src/bits.rs
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
    RuntimeError: bundler is not specified: src/int/inv.rs\n"
  code: "pub use super::*;\n\npub fn inv<I: Int>(a: I, modu: I) -> I {\n    let [zero,\
    \ one]: [I::Signed; 2] = [I::Signed::ZERO, I::Signed::ONE];\n    let [mut a, mut\
    \ b, mut u, mut v]: [I::Signed; 4] = [a.as_(), modu.as_(), one, zero];\n    while\
    \ b != zero {\n        let t = a / b;\n        a -= t * b;\n        u -= t * v;\n\
    \        std::mem::swap(&mut a, &mut b);\n        std::mem::swap(&mut u, &mut\
    \ v);\n    }\n    debug_assert_eq!(a, one);\n    if u < zero {\n        debug_assert_eq!(v,\
    \ modu.as_());\n        debug_assert!(u > zero);\n        u += v;\n    }\n   \
    \ a.as_()\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/inv.rs
  requiredBy: []
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/inv.rs
layout: document
redirect_from:
- /library/src/int/inv.rs
- /library/src/int/inv.rs.html
title: src/int/inv.rs
---
