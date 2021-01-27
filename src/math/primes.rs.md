---
data:
  _extendedDependsOn:
  - icon: ':warning:'
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
  code: "use crate::ds::bitset::*;\n\npub fn primes(n: u32) -> Vec<u32> {\n\tlet n\
    \ = n as usize;\n\tlet mut prime = new_bitset(n + 1);\n\tprime.negate();\n\tfor\
    \ p in 2..=n {\n\t\tif prime.get_bit(p) {\n\t\t\tfor j in ((p * 2)..=n).step_by(p)\
    \ {\n\t\t\t\tprime.set_bit(j, false);\n\t\t\t}\n\t\t}\n\t}\n\t(2..=n as u32).filter(|&i|\
    \ prime.get_bit(i as usize)).collect()\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  isVerificationFile: false
  path: src/math/primes.rs
  requiredBy: []
  timestamp: '2021-01-03 22:19:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/primes.rs
layout: document
redirect_from:
- /library/src/math/primes.rs
- /library/src/math/primes.rs.html
title: src/math/primes.rs
---
