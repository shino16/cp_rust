---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub struct Rng32(u32);\n\nimpl Rng32 {\n    pub fn new() -> Self {\n    \
    \    Rng32(2_463_534_242)\n    }\n    pub fn rand(&mut self) -> u32 {\n      \
    \  let mut x = self.0;\n        x ^= x << 13;\n        x ^= x >> 17;\n       \
    \ x ^= x << 5;\n        self.0 = x;\n        x\n    }\n}\n\npub struct Rng64(u64);\n\
    \nimpl Rng64 {\n    pub fn new() -> Self {\n        Rng64(88_172_645_463_325_252)\n\
    \    }\n    pub fn rand(&mut self) -> u64 {\n        let mut x = self.0;\n   \
    \     x ^= x << 13;\n        x ^= x >> 7;\n        x ^= x << 17;\n        self.0\
    \ = x;\n        x\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rng.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-10-18 23:49:38+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/rng.rs
layout: document
redirect_from:
- /library/src/rng.rs
- /library/src/rng.rs.html
title: src/rng.rs
---
