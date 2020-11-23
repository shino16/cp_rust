---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':x:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "#[inline(always)]\npub fn modpow(e: u32, k: u32, m: u32) -> u32 {\n    let\
    \ (mut e, mut k, m) = (e as u64, k as u64, m as u64);\n    let mut res = 1;\n\
    \    while k != 0 {\n        if k % 2 != 0 {\n            res = res * e % m;\n\
    \        }\n        e = e * e % m;\n        k /= 2;\n    }\n    res as u32\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/modpow.rs
  requiredBy: []
  timestamp: '2020-11-17 18:39:28+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/math/modpow.rs
layout: document
redirect_from:
- /library/src/math/modpow.rs
- /library/src/math/modpow.rs.html
title: src/math/modpow.rs
---
