---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/poly.rs
    title: src/poly.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/num/field.rs\n"
  code: 'pub use super::*;


    pub trait Field: Num + Neg<Output = Self> + Div<Self, Output = Self> + DivAssign<Self>
    {}

    impl<T: Num + Neg<Output = Self> + Div<Self, Output = Self> + DivAssign<Self>>
    Field for T {}

    '
  dependsOn:
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/num/field.rs
  requiredBy:
  - src/poly.rs
  timestamp: '2021-02-13 20:22:55+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/num/field.rs
layout: document
redirect_from:
- /library/src/num/field.rs
- /library/src/num/field.rs.html
title: src/num/field.rs
---
