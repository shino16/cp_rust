---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/conv.rs
    title: src/conv.rs
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
    RuntimeError: bundler is not specified: src/poly.rs\n"
  code: "pub use crate::conv::*;\npub use crate::num::field::*;\n\npub trait Poly:\
    \ Field + Conv {}\nimpl<T: Field + Conv> Poly for T {}\n\n// TODO\n\npub fn inv<T:\
    \ Poly>(f: Vec<T>, need: usize) -> Vec<T> {\n    let (mut f2, mut inv2) = (Vec::new(),\
    \ Vec::new());\n    let mut inv = vec![T::one() / f[0]];\n    let mut deg = 1;\n\
    \    while deg < need {\n        deg *= 2;\n        f2.clone_from(&f);\n     \
    \   inv2.clone_from(&inv);\n        f2.truncate(deg);\n        Conv::conv_in_place(&mut\
    \ f2, &mut inv2);\n        f2.truncate(deg);\n        for e in &mut f2 {\n   \
    \         *e = -*e;\n        }\n        f2[0] += T::one() + T::one();\n      \
    \  Conv::conv_in_place(&mut inv, &mut f2);\n        inv.truncate(deg);\n    }\n\
    \    inv\n}"
  dependsOn:
  - src/conv.rs
  - src/num.rs
  - src/num/field.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/poly.rs
  requiredBy: []
  timestamp: '2021-05-07 12:42:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/poly.rs
layout: document
redirect_from:
- /library/src/poly.rs
- /library/src/poly.rs.html
title: src/poly.rs
---
