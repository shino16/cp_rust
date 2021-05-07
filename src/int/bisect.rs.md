---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
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
    RuntimeError: bundler is not specified: src/int/bisect.rs\n"
  code: "use crate::bits::*;\nuse crate::zo::*;\n\npub fn bisect<I>(mut l: I, mut\
    \ r: I, mut pred: impl FnMut(I) -> bool) -> I\nwhere\n    I: Bits + ZeroOne +\
    \ std::ops::Add<I, Output = I> + std::fmt::Debug,\n{\n    while l != r {\n   \
    \     let mid = (l & r) + ((l ^ r) >> 1);\n        if pred(mid) {\n          \
    \  l = mid + I::ONE;\n        } else {\n            r = mid;\n        }\n    }\n\
    \    r\n}\n"
  dependsOn:
  - src/bits.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/bisect.rs
  requiredBy: []
  timestamp: '2021-04-26 15:43:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/bisect.rs
layout: document
redirect_from:
- /library/src/int/bisect.rs
- /library/src/int/bisect.rs.html
title: src/int/bisect.rs
---
