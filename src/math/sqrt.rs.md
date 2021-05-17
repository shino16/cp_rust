---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/sqrt.rs\n"
  code: "type UInt = u64;\n\npub fn floor_sqrt(n: UInt) -> UInt {\n    if n == 0 {\n\
    \        0\n    } else {\n        let x = (n as f64).sqrt().round() as UInt;\n\
    \        (x + n / x) / 2\n    }\n}\n\npub fn ceil_sqrt(n: UInt) -> UInt {\n  \
    \  if n == 0 {\n        0\n    } else {\n        floor_sqrt(n - 1) + 1\n    }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/sqrt.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-05-17 15:14:26+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/math/sqrt.rs
layout: document
redirect_from:
- /library/src/math/sqrt.rs
- /library/src/math/sqrt.rs.html
title: src/math/sqrt.rs
---
