---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/math/primes.rs\n"
  code: "use crate::ds::bitset::*;\n\npub fn primes(n: u32) -> impl Iterator<Item\
    \ = u32> {\n    let n = n as usize;\n    let mut prime = new_bitset(n + 1);\n\
    \    prime.negate();\n    for p in 2..=n {\n        if prime.get_bit(p) {\n  \
    \          for j in ((p * 2)..=n).step_by(p) {\n                prime.set_bit(j,\
    \ false);\n            }\n        }\n    }\n    struct Iter(usize, usize, Vec<u32>);\n\
    \    impl Iterator for Iter {\n        type Item = u32;\n        fn next(&mut\
    \ self) -> Option<u32> {\n            while self.0 < self.1 {\n              \
    \  self.0 += 1;\n                if self.2.get_bit(self.0) {\n               \
    \     return Some(self.0 as u32);\n                }\n            }\n        \
    \    None\n        }\n    }\n    Iter(1, n, prime)\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  isVerificationFile: false
  path: src/math/primes.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/primes.rs
layout: document
redirect_from:
- /library/src/math/primes.rs
- /library/src/math/primes.rs.html
title: src/math/primes.rs
---
