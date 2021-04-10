---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/modpow.rs\n"
  code: "#[inline(always)]\npub fn modpow(e: u64, mut k: u64, modu: u64) -> u64 {\n\
    \    let (mut e, modu) = (e as u64, modu as u64);\n    let mut res = 1;\n    while\
    \ k != 0 {\n        if k % 2 != 0 {\n            res = res * e % modu;\n     \
    \   }\n        e = e * e % modu;\n        k /= 2;\n    }\n    res as u64\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math/modpow.rs
  requiredBy: []
  timestamp: '2021-03-03 02:01:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/modpow.rs
layout: document
redirect_from:
- /library/src/math/modpow.rs
- /library/src/math/modpow.rs.html
title: src/math/modpow.rs
---
