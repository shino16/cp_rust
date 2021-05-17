---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/func/rec.rs\n"
  code: "pub fn recurse<A, R, F>(f: F) -> impl Fn(A) -> R\nwhere\n    F: Fn(&dyn Fn(A)\
    \ -> R, A) -> R,\n{\n    fn fix<A, R, F>(f: &F, a: A) -> R\n    where\n      \
    \  F: Fn(&dyn Fn(A) -> R, A) -> R,\n    {\n        f(&|a: A| fix::<A, R, F>(f,\
    \ a), a)\n    }\n    move |a: A| fix::<A, R, F>(&f, a)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/func/rec.rs
  requiredBy: []
  timestamp: '2021-05-17 11:32:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/func/rec.rs
layout: document
redirect_from:
- /library/src/func/rec.rs
- /library/src/func/rec.rs.html
title: src/func/rec.rs
---
