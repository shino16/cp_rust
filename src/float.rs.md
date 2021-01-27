---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/float/bisect.rs
    title: src/float/bisect.rs
  - icon: ':warning:'
    path: src/float/convex.rs
    title: src/float/convex.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/float.rs\n"
  code: 'pub mod bisect;

    pub mod convex;


    pub type Float = f64;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/float.rs
  requiredBy:
  - src/float/bisect.rs
  - src/float/convex.rs
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float.rs
layout: document
redirect_from:
- /library/src/float.rs
- /library/src/float.rs.html
title: src/float.rs
---
