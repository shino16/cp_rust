---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/gf/dynamic.rs
    title: src/gf/dynamic.rs
  - icon: ':warning:'
    path: src/math/crt.rs
    title: src/math/crt.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/gcd/ext.rs\n"
  code: "type Int = u64;\ntype IInt = i64;\n\n/// (g, x) where g = gcd(a, b), ax =\
    \ g (mod b), 0 <= x < b/g\npub fn extgcd(a: Int, b: Int) -> (Int, Int) {\n   \
    \ let (mut a, mut b) = (a as IInt, b as IInt);\n    let b0 = b;\n    // A = [a,\
    \ x, y; b, u, v], k = [-1; a; b], Ak = 0\n    let (mut x, mut u) = (1, 0);\n \
    \   while b != 0 {\n        let t = a / b;\n        a -= t * b;\n        x -=\
    \ t * u;\n        std::mem::swap(&mut a, &mut b);\n        std::mem::swap(&mut\
    \ x, &mut u);\n    }\n    if x < 0 {\n        x += b0 / a;\n    }\n    (a as Int,\
    \ x as Int)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/gcd/ext.rs
  requiredBy:
  - src/math/crt.rs
  - src/gf/dynamic.rs
  timestamp: '2021-02-28 10:03:54+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/gcd/ext.rs
layout: document
redirect_from:
- /library/src/math/gcd/ext.rs
- /library/src/math/gcd/ext.rs.html
title: src/math/gcd/ext.rs
---
