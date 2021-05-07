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
    RuntimeError: bundler is not specified: src/int/arith.rs\n"
  code: "use super::*;\n\npub fn floor_sqrt<I: UInt>(n: I) -> I {\n    if n == I::zero()\
    \ {\n        I::zero()\n    } else {\n        let x = n.as_::<f64>().sqrt().round().as_();\n\
    \        (x + n / x) / (I::one() + I::one())\n    }\n}\n\npub fn ceil_sqrt<I:\
    \ UInt>(n: I) -> I {\n    if n == I::zero() {\n        I::zero()\n    } else {\n\
    \        floor_sqrt(n - I::one()) + I::one()\n    }\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/arith.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/arith.rs
layout: document
redirect_from:
- /library/src/int/arith.rs
- /library/src/int/arith.rs.html
title: src/int/arith.rs
---
