---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {\n\tlet mut res = Vec::new();\n\
    \tif n % 2 != 0 {\n\t\tlet t = n.trailing_zeros();\n\t\tres.push((2, t));\n\t\t\
    n >>= t;\n\t}\n\tfor d in (3..).step_by(2) {\n\t\tif d * d > n {\n\t\t\tbreak;\n\
    \t\t}\n\t\tif n % d == 0 {\n\t\t\tlet mut cnt = 1;\n\t\t\tn /= d;\n\t\t\twhile\
    \ n % d == 0 {\n\t\t\t\tcnt += 1;\n\t\t\t\tn /= d;\n\t\t\t}\n\t\t\tres.push((d,\
    \ cnt));\n\t\t}\n\t}\n\tres\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/factorize.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/factorize.rs
layout: document
redirect_from:
- /library/src/math/factorize.rs
- /library/src/math/factorize.rs.html
title: src/math/factorize.rs
---
