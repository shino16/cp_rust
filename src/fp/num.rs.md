---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
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
    RuntimeError: bundler is not specified: src/fp/num.rs\n"
  code: "pub use super::*;\nuse crate::num::*;\n\nimpl<M: Mod> Num for Fp<M> {\n\t\
    fn wrapping_add(self, rhs: Self) -> Self {\n\t\tself + rhs\n\t}\n\tfn wrapping_neg(self)\
    \ -> Self {\n\t\t-self\n\t}\n}\n"
  dependsOn:
  - src/fp.rs
  - src/io.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/fp/num.rs
  requiredBy: []
  timestamp: '2021-01-30 12:54:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/fp/num.rs
layout: document
redirect_from:
- /library/src/fp/num.rs
- /library/src/fp/num.rs.html
title: src/fp/num.rs
---
