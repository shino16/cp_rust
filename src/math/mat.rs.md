---
data:
  _extendedDependsOn:
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/mat.rs\n"
  code: "use crate::num::*;\n\npub type Mat<T> = Vec<Vec<T>>;\n\npub fn eye<T: Ring>(n:\
    \ usize) -> Mat<T> {\n    let mut res = vec![vec![T::ZERO; n]; n];\n    for i\
    \ in 0..n {\n        res[i][i] = T::ONE;\n    }\n    res\n}\n\npub fn mat_mul<T:\
    \ Ring>(a: &Mat<T>, b: &Mat<T>) -> Mat<T> {\n    let mut res = vec![vec![T::ZERO;\
    \ b[0].len()]; a.len()];\n    for (res, a) in res.iter_mut().zip(a) {\n      \
    \  for (&a, b) in a.iter().zip(b) {\n            for (res, &b) in res.iter_mut().zip(b)\
    \ {\n                *res += a * b;\n            }\n        }\n    }\n    res\n\
    }\n\npub fn mat_pow<T: Ring>(mut e: Mat<T>, mut k: u64) -> Mat<T> {\n    let mut\
    \ res = eye(e.len());\n    while k != 0 {\n        if k % 2 != 0 {\n         \
    \   res = mat_mul(&res, &e);\n        }\n        e = mat_mul(&e, &e);\n      \
    \  k /= 2;\n    }\n    res\n}\n"
  dependsOn:
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/mat.rs
  requiredBy: []
  timestamp: '2021-04-26 15:43:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/mat.rs
layout: document
redirect_from:
- /library/src/math/mat.rs
- /library/src/math/mat.rs.html
title: src/math/mat.rs
---
