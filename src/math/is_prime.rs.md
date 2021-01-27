---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/u64.rs
    title: src/u64.rs
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
    RuntimeError: bundler is not specified: src/math/is_prime.rs\n"
  code: "use crate::u64::*;\n\n/// n < 7e18\npub fn is_prime(n: u64) -> bool {\n\t\
    if n < 2 {\n\t\tfalse\n\t} else if n % 6 % 4 != 1 {\n\t\tn == 2 || n == 3\n\t\
    } else {\n\t\tlet s = (n - 1).trailing_zeros();\n\t\tfor &a in &[2, 325, 9375,\
    \ 28178, 450775, 9780504, 1795265022] {\n\t\t\tlet mut p = modpow64(a % n, n >>\
    \ s, n);\n\t\t\tlet mut i = s;\n\t\t\twhile p != 1 && p != n - 1 && a % n != 0\
    \ && i != 0 {\n\t\t\t\tp = modmul64(p, p, n);\n\t\t\t\ti -= 1;\n\t\t\t}\n\t\t\t\
    if p != n - 1 && i != s {\n\t\t\t\treturn false;\n\t\t\t}\n\t\t}\n\t\ttrue\n\t\
    }\n}\n"
  dependsOn:
  - src/u64.rs
  isVerificationFile: false
  path: src/math/is_prime.rs
  requiredBy: []
  timestamp: '2021-01-13 14:07:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/is_prime.rs
layout: document
redirect_from:
- /library/src/math/is_prime.rs
- /library/src/math/is_prime.rs.html
title: src/math/is_prime.rs
---