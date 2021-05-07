---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':warning:'
    path: src/num/field.rs
    title: src/num/field.rs
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
    RuntimeError: bundler is not specified: src/math/bm.rs\n"
  code: "use crate::num::field::*;\n\npub fn berlekamp_massey<T: Field>(s: &[T]) ->\
    \ Vec<T> {\n    let (mut c, mut b) = (vec![T::zero(); s.len()], vec![T::zero();\
    \ s.len()]);\n    c[0] = T::one();\n    b[0] = T::one();\n    let mut db = T::one();\n\
    \    let mut m = 0;\n    for n in 0..s.len() {\n        m += 1;\n        let dc\
    \ = (0..=n).fold(T::zero(), |d, i| d + c[i] * s[n - i]);\n        if dc != T::zero()\
    \ {\n            let k = dc / db;\n            for i in (m..s.len()).rev() {\n\
    \                b[i] = c[i] - k * b[i - m];\n            }\n            b[..m].copy_from_slice(&c[..m]);\n\
    \            std::mem::swap(&mut b, &mut c);\n            db = dc;\n         \
    \   m = 0;\n        }\n    }\n    while c.last() == Some(&T::zero()) {\n     \
    \   c.pop();\n    }\n    c.remove(0);\n    c.iter_mut().for_each(|c| *c = -*c);\n\
    \    c\n}\n"
  dependsOn:
  - src/num.rs
  - src/num/field.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/math/bm.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/bm.rs
layout: document
redirect_from:
- /library/src/math/bm.rs
- /library/src/math/bm.rs.html
title: src/math/bm.rs
---
