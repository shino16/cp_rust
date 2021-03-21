---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/pow.rs\n"
  code: "use crate::int::*;\n\n#[inline(always)]\npub fn pow<T: Num, K: UInt>(mut\
    \ e: T, mut k: K) -> T {\n    let mut res = T::ONE;\n    let two = K::ONE + K::ONE;\n\
    \    while k != K::ZERO {\n        if k % two != K::ZERO {\n            res *=\
    \ e;\n        }\n        e *= e;\n        k /= two;\n    }\n    res\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/pow.rs
  requiredBy: []
  timestamp: '2021-03-22 00:48:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/pow.rs
layout: document
redirect_from:
- /library/src/math/pow.rs
- /library/src/math/pow.rs.html
title: src/math/pow.rs
---
