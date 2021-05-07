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
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/int/gcd/ext.rs\n"
  code: "pub use crate::int::*;\n\n/// (g, x) where g = gcd(a, b), ax = g (mod b),\
    \ 0 <= x < b/g\npub fn extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I) {\n    //\
    \ A = [a, x, y; b, u, v], k = [-1; a; b], Ak = 0\n    let (mut x, mut u) = (I::one(),\
    \ I::zero());\n    let b0 = b;\n    while !b.is_zero() {\n        let t = a /\
    \ b;\n        a -= t * b;\n        x -= t * u;\n        std::mem::swap(&mut a,\
    \ &mut b);\n        std::mem::swap(&mut x, &mut u);\n    }\n    if x < I::zero()\
    \ {\n        x += b0 / a;\n    }\n    (a, x)\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/gcd/ext.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/gcd/ext.rs
layout: document
redirect_from:
- /library/src/int/gcd/ext.rs
- /library/src/int/gcd/ext.rs.html
title: src/int/gcd/ext.rs
---
