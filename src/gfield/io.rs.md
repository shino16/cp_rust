---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/gfield.rs
    title: src/gfield.rs
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
    RuntimeError: bundler is not specified: src/gfield/io.rs\n"
  code: "pub use super::*;\nuse crate::io::*;\n\nimpl<M: Mod> Print for GField<M>\
    \ {\n    fn print(w: &mut IO, x: Self) { w.print(x.value()); }\n}\nimpl<M: Mod>\
    \ Scan for GField<M> {\n    fn scan(io: &mut IO) -> Self { Self::new(io.scan())\
    \ }\n}\n"
  dependsOn:
  - src/gfield.rs
  - src/io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/gfield/io.rs
  requiredBy: []
  timestamp: '2021-03-25 23:36:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/gfield/io.rs
layout: document
redirect_from:
- /library/src/gfield/io.rs
- /library/src/gfield/io.rs.html
title: src/gfield/io.rs
---
