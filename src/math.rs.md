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
    RuntimeError: bundler is not specified: src/math.rs\n"
  code: 'pub mod binom;

    pub mod bm;

    pub mod convex;

    pub mod crt;

    pub mod factorize;

    pub mod gcd;

    pub mod is_prime;

    pub mod modpow;

    pub mod pow;

    pub mod primes;

    '
  dependsOn: []
  isVerificationFile: false
  path: src/math.rs
  requiredBy: []
  timestamp: '2021-04-17 01:05:25+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math.rs
layout: document
redirect_from:
- /library/src/math.rs
- /library/src/math.rs.html
title: src/math.rs
---
