---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use super::*;\n\npub fn floor_sqrt<I: UInt>(n: I) -> I {\n\tif n == I::ZERO\
    \ {\n\t\tI::ZERO\n\t} else {\n\t\tlet x = n.as_::<f64>().sqrt().round().as_();\n\
    \t\t(x + n / x) >> 1\n\t}\n}\n\npub fn ceil_sqrt<I: UInt>(n: I) -> I {\n\tif n\
    \ == I::ZERO {\n\t\tI::ZERO\n\t} else {\n\t\tfloor_sqrt(n - I::ONE) + I::ONE\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/int/arith.rs
  requiredBy: []
  timestamp: '2020-12-21 16:41:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/int/arith.rs
layout: document
redirect_from:
- /library/src/int/arith.rs
- /library/src/int/arith.rs.html
title: src/int/arith.rs
---
