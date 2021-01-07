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
  code: "#[inline(always)]\npub fn modpow(e: u32, mut k: u64, modu: u32) -> u32 {\n\
    \tlet (mut e, modu) = (e as u64, modu as u64);\n\tlet mut res = 1;\n\twhile k\
    \ != 0 {\n\t\tif k % 2 != 0 {\n\t\t\tres = res * e % modu;\n\t\t}\n\t\te = e *\
    \ e % modu;\n\t\tk /= 2;\n\t}\n\tres as u32\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/modpow.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/modpow.rs
layout: document
redirect_from:
- /library/src/math/modpow.rs
- /library/src/math/modpow.rs.html
title: src/math/modpow.rs
---
