---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/iter/either.rs\n"
  code: "pub enum Either<L, R> {\n    Left(L),\n    Right(R),\n}\n\nimpl<A, L, R>\
    \ Iterator for Either<L, R>\nwhere\n    L: Iterator<Item = A>,\n    R: Iterator<Item\
    \ = A>,\n{\n    type Item = A;\n    fn next(&mut self) -> Option<Self::Item> {\n\
    \        match self {\n            Self::Left(l) => l.next(),\n            Self::Right(r)\
    \ => r.next(),\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/iter/either.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/iter/either.rs
layout: document
redirect_from:
- /library/src/iter/either.rs
- /library/src/iter/either.rs.html
title: src/iter/either.rs
---
