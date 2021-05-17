---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math.rs\n"
  code: "pub mod binom;\npub mod bm;\npub mod convex;\npub mod crt;\npub mod factorize;\n\
    pub mod gcd;\npub mod is_prime;\npub mod mat;\npub mod modpow;\npub mod pow;\n\
    pub mod primes;\npub mod sqrt;\n\npub fn abs_diff<T: PartialOrd + std::ops::Sub<T,\
    \ Output = T>>(a: T, b: T) -> T {\n    if a < b { b - a } else { a - b }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/math.rs
  requiredBy: []
  timestamp: '2021-05-17 15:14:26+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math.rs
layout: document
redirect_from:
- /library/src/math.rs
- /library/src/math.rs.html
title: src/math.rs
---
