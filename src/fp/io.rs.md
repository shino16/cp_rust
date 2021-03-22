---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/fp/io.rs\n"
  code: "pub use super::*;\nuse crate::io::*;\n\nimpl<M: Mod> Print for Fp<M> {\n\
    \    fn print(w: &mut IO, x: Self) { w.print(x.value()); }\n}\nimpl<M: Mod> Scan\
    \ for Fp<M> {\n    fn scan(io: &mut IO) -> Self { Self::new(io.scan()) }\n}\n"
  dependsOn:
  - src/fp.rs
  - src/io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/fp/io.rs
  requiredBy: []
  timestamp: '2021-03-22 00:48:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/fp/io.rs
layout: document
redirect_from:
- /library/src/fp/io.rs
- /library/src/fp/io.rs.html
title: src/fp/io.rs
---
