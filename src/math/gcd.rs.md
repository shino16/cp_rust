---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/gcd.rs\n"
  code: "type Int = i64;\ntype UInt = u64;\n\npub mod ext;\n\npub fn gcd(a: Int, b:\
    \ Int) -> Int { ugcd(a.abs() as _, b.abs() as _) as _ }\npub fn lcm(a: Int, b:\
    \ Int) -> Int { a / gcd(a, b) * b }\n\npub fn ugcd(mut a: UInt, mut b: UInt) ->\
    \ UInt {\n    if a == 0 {\n        return b;\n    } else if b == 0 {\n       \
    \ return a;\n    }\n    let a_shift = a.trailing_zeros();\n    a >>= a_shift;\n\
    \    let b_shift = b.trailing_zeros();\n    b >>= b_shift;\n    while a != b {\n\
    \        if a > b {\n            std::mem::swap(&mut a, &mut b);\n        }\n\
    \        b -= a;\n        b >>= b.trailing_zeros();\n    }\n    a << a_shift.min(b_shift)\n\
    }\npub fn ulcm(a: UInt, b: UInt) -> UInt { a / ugcd(a, b) * b }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/gcd.rs
  requiredBy: []
  timestamp: '2021-04-16 00:20:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/gcd.rs
layout: document
redirect_from:
- /library/src/math/gcd.rs
- /library/src/math/gcd.rs.html
title: src/math/gcd.rs
---
