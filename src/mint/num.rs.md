---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
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
    RuntimeError: bundler is not specified: src/mint/num.rs\n"
  code: "pub use super::*;\nuse crate::num::*;\n\nimpl<M: Mod> Num for Mint<M> {\n\
    \    fn wrapping_add(self, rhs: Self) -> Self {\n        self + rhs\n    }\n \
    \   fn wrapping_neg(self) -> Self {\n        -self\n    }\n}\n"
  dependsOn:
  - src/mint.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/mint/num.rs
  requiredBy: []
  timestamp: '2021-02-08 23:15:08+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/mint/num.rs
layout: document
redirect_from:
- /library/src/mint/num.rs
- /library/src/mint/num.rs.html
title: src/mint/num.rs
---
