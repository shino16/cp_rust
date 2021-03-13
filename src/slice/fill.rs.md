---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/slice/fill.rs\n"
  code: "pub trait Fill {\n    type Item: Clone;\n    fn fill(&mut self, val: Self::Item);\n\
    }\n\nimpl<T: Clone> Fill for [T] {\n    type Item = T;\n    fn fill(&mut self,\
    \ val: Self::Item) {\n        for e in self {\n            *e = val.clone();\n\
    \        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/fill.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice/fill.rs
layout: document
redirect_from:
- /library/src/slice/fill.rs
- /library/src/slice/fill.rs.html
title: src/slice/fill.rs
---
