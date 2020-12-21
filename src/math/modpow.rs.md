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
  code: "#[inline(always)]\npub fn modpow(e: u32, k: u32, m: u32) -> u32 {\n\tlet\
    \ (mut e, mut k, m) = (e as u64, k as u64, m as u64);\n\tlet mut res = 1;\n\t\
    while k != 0 {\n\t\tif k % 2 != 0 {\n\t\t\tres = res * e % m;\n\t\t}\n\t\te =\
    \ e * e % m;\n\t\tk /= 2;\n\t}\n\tres as u32\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/modpow.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/modpow.rs
layout: document
redirect_from:
- /library/src/math/modpow.rs
- /library/src/math/modpow.rs.html
title: src/math/modpow.rs
---
