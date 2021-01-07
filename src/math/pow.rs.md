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
  code: "use crate::int::*;\n\n#[inline(always)]\npub fn pow<T: Num, K: UInt>(mut\
    \ e: T, mut k: K) -> T {\n\tlet mut res = T::ONE;\n\twhile k != K::ZERO {\n\t\t\
    if k & K::ONE != K::ZERO {\n\t\t\tres = res * e;\n\t\t}\n\t\te = e * e;\n\t\t\
    k >>= 1;\n\t}\n\tres\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/pow.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/pow.rs
layout: document
redirect_from:
- /library/src/math/pow.rs
- /library/src/math/pow.rs.html
title: src/math/pow.rs
---
