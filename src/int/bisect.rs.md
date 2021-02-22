---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
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
    RuntimeError: bundler is not specified: src/int/bisect.rs\n"
  code: "use super::*;\n\npub fn bisect<I: Int, F: FnMut(I) -> bool>(mut l: I, mut\
    \ r: I, mut pred: F) -> I {\n    let two = I::ONE + I::ONE;\n    while l != r\
    \ {\n        let mid = (l + r) / two;\n        if pred(mid) {\n            l =\
    \ mid + I::ONE;\n        } else {\n            r = mid;\n        }\n    }\n  \
    \  r\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int/bisect.rs
  requiredBy: []
  timestamp: '2021-02-16 22:10:53+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/bisect.rs
layout: document
redirect_from:
- /library/src/int/bisect.rs
- /library/src/int/bisect.rs.html
title: src/int/bisect.rs
---
