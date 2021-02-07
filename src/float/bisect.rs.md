---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/float.rs
    title: src/float.rs
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
    RuntimeError: bundler is not specified: src/float/bisect.rs\n"
  code: "use super::*;\n\npub fn bisect<F: FnMut(Float) -> bool>(\n    mut l: Float,\n\
    \    mut r: Float,\n    e: Float,\n    mut pred: F,\n) -> Float {\n    let k =\
    \ ((r - l) / e).log2() as u32 + 2;\n    for _ in 0..k {\n        let mid = (l\
    \ + r) / 2.0;\n        if pred(mid) {\n            l = mid;\n        } else {\n\
    \            r = mid;\n        }\n    }\n    r\n}\n"
  dependsOn:
  - src/float.rs
  isVerificationFile: false
  path: src/float/bisect.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/float/bisect.rs
layout: document
redirect_from:
- /library/src/float/bisect.rs
- /library/src/float/bisect.rs.html
title: src/float/bisect.rs
---
