---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/ds/idx.rs
    title: src/ds/idx.rs
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
    RuntimeError: bundler is not specified: src/math/primes.rs\n"
  code: "use crate::ds::idx::*;\n\npub fn prime_table(upto: u32) -> IdxInt<bool> {\n\
    \    let mut pr = IdxInt(vec![true; upto as usize + 1]);\n    pr[0] = false;\n\
    \    pr[1] = false;\n    for i in 2..=upto {\n        if pr[i] {\n           \
    \ for j in (i + i..=upto).step_by(i as usize) {\n                pr[j] = false;\n\
    \            }\n        }\n    }\n    pr\n}\n\npub fn primes(upto: u32) -> Vec<u32>\
    \ {\n    let is_prime = prime_table(upto);\n    (2..=upto)\n        .filter(|&n|\
    \ is_prime[n])\n        .map(|n| n as u32)\n        .collect()\n}\n"
  dependsOn:
  - src/ds/idx.rs
  isVerificationFile: false
  path: src/math/primes.rs
  requiredBy: []
  timestamp: '2021-04-26 15:43:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/primes.rs
layout: document
redirect_from:
- /library/src/math/primes.rs
- /library/src/math/primes.rs.html
title: src/math/primes.rs
---
