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
    RuntimeError: bundler is not specified: src/util/abs_diff.rs\n"
  code: "pub fn abs_diff<T: Ord + std::ops::Sub<T, Output = T>>(a: T, b: T) -> T {\n\
    \    if a > b { a - b } else { b - a }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/util/abs_diff.rs
  requiredBy: []
  timestamp: '2021-03-06 13:39:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/util/abs_diff.rs
layout: document
redirect_from:
- /library/src/util/abs_diff.rs
- /library/src/util/abs_diff.rs.html
title: src/util/abs_diff.rs
---
