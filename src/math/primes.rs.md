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
    \ = u32> {\n\tlet n = n as usize;\n\tlet mut prime = new_bitset(n + 1);\n\tprime.negate();\n\
    \tfor p in 2..=n {\n\t\tif prime.get_bit(p) {\n\t\t\tfor j in ((p * 2)..=n).step_by(p)\
    \ {\n\t\t\t\tprime.set_bit(j, false);\n\t\t\t}\n\t\t}\n\t}\n\tstruct Iter(usize,\
    \ usize, Vec<u32>);\n\timpl Iterator for Iter {\n\t\ttype Item = u32;\n\t\tfn\
    \ next(&mut self) -> Option<u32> {\n\t\t\twhile self.0 < self.1 {\n\t\t\t\tself.0\
    \ += 1;\n\t\t\t\tif self.2.get_bit(self.0) {\n\t\t\t\t\treturn Some(self.0 as\
    \ u32);\n\t\t\t\t}\n\t\t\t}\n\t\t\tNone\n\t\t}\n\t}\n\tIter(1, n, prime)\n}\n"
  dependsOn:
  - src/ds/bitset.rs
  isVerificationFile: false
  path: src/math/primes.rs
  requiredBy: []
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/math/primes.rs
layout: document
redirect_from:
- /library/src/math/primes.rs
- /library/src/math/primes.rs.html
title: src/math/primes.rs
---
