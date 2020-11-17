---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::num::*;\n\n// binary gcd\npub fn gcd<I: Int>(mut a: I, mut b:\
    \ I) -> I {\n    if a.is_zero() {\n        return b;\n    } else if b.is_zero()\
    \ {\n        return a;\n    }\n    let a_shift = a.trailing_zeros();\n    a >>=\
    \ a_shift;\n    let b_shift = b.trailing_zeros();\n    b >>= b_shift;\n    let\
    \ shift = a_shift.min(b_shift);\n    loop {\n        if a > b {\n            std::mem::swap(&mut\
    \ a, &mut b);\n        }\n        b -= a;\n        if b.is_zero() {\n        \
    \    return a << shift;\n        }\n        b >>= b.trailing_zeros();\n    }\n\
    }\n\n// (x, y, g) where ax + by = g\npub fn extgcd<I: IInt>(mut a: I, mut b: I)\
    \ -> (I, I, I) {\n    let (mut x, mut y, mut u, mut v) = (I::ONE, I::ZERO, I::ZERO,\
    \ I::ONE);\n    // Euclidean algorithm by elementary row operations on A_0 = [a,\
    \ x, y; b, u, v]\n    // invariant: Ax = 0 where x = [-1, a, b]\n    while !b.is_zero()\
    \ {\n        let t = a / b;\n        a -= t * b;\n        x -= t * u;\n      \
    \  y -= t * v;\n        std::mem::swap(&mut a, &mut b);\n        std::mem::swap(&mut\
    \ x, &mut u);\n        std::mem::swap(&mut y, &mut v);\n    }\n    (x, y, a)\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/gcd.rs
  requiredBy: []
  timestamp: '2020-11-17 16:16:39+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/math/gcd.rs
layout: document
redirect_from:
- /library/src/math/gcd.rs
- /library/src/math/gcd.rs.html
title: src/math/gcd.rs
---