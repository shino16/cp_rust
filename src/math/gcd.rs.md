---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/math/gcd.rs\n"
  code: "type Int = i32;\ntype UInt = u64;\n\npub fn gcd(a: Int, b: Int) -> Int {\n\
    \    ugcd(a.abs() as _, b.abs() as _) as _\n}\n\n// binary gcd\npub fn ugcd(a:\
    \ UInt, b: UInt) -> UInt {\n    #[target_feature(enable = \"bmi1\")]\n    unsafe\
    \ fn ugcd_impl(mut a: UInt, mut b: UInt) -> UInt {\n        if a == 0 {\n    \
    \        return b;\n        } else if b == 0 {\n            return a;\n      \
    \  }\n        let a_shift = a.trailing_zeros();\n        a >>= a_shift;\n    \
    \    let b_shift = b.trailing_zeros();\n        b >>= b_shift;\n        while\
    \ a != b {\n            if a > b {\n                std::mem::swap(&mut a, &mut\
    \ b);\n            }\n            b -= a;\n            b >>= b.trailing_zeros();\n\
    \        }\n        a << a_shift.min(b_shift)\n    }\n    unsafe {\n        ugcd_impl(a,\
    \ b)\n    }\n}\n\n/// (x, y, g) where ax + by = g, x >= 0\npub fn extgcd(mut a:\
    \ Int, mut b: Int) -> (Int, Int, Int) {\n    // A = [a, x, y; b, u, v], k = [-1;\
    \ a0; b0]\n    // A'= [a, x, y; 0, u, v] \\therefore a0*u + b0*v = 0\n    let\
    \ (mut x, mut y, mut u, mut v) = (1, 0, 0, 1);\n    while b != 0 {\n        let\
    \ t = a / b;\n        a -= t * b;\n        x -= t * u;\n        y -= t * v;\n\
    \        std::mem::swap(&mut a, &mut b);\n        std::mem::swap(&mut x, &mut\
    \ u);\n        std::mem::swap(&mut y, &mut v);\n    }\n    if x < 0 {\n      \
    \  x += u;\n        y -= v;\n        debug_assert_eq!(gcd(u, v), 1);\n       \
    \ debug_assert!(x + u >= 0);\n    }\n    (x, y, a)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/gcd.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/gcd.rs
layout: document
redirect_from:
- /library/src/math/gcd.rs
- /library/src/math/gcd.rs.html
title: src/math/gcd.rs
---