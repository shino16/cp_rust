---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/rand/xorshift.rs\n"
  code: "pub struct Xorshift32(u32);\n\nimpl Xorshift32 {\n    pub fn new() -> Self\
    \ {\n        Self(2_463_534_242)\n    }\n    pub fn next(&mut self) -> u32 {\n\
    \        let mut x = self.0;\n        x ^= x << 13;\n        x ^= x >> 17;\n \
    \       x ^= x << 5;\n        self.0 = x;\n        x\n    }\n}\n\npub struct Xorshift64(u64);\n\
    \nimpl Xorshift64 {\n    pub fn new() -> Self {\n        Self(88_172_645_463_325_252)\n\
    \    }\n    pub fn next(&mut self) -> u64 {\n        let mut x = self.0;\n   \
    \     x ^= x << 13;\n        x ^= x >> 7;\n        x ^= x << 17;\n        self.0\
    \ = x;\n        x\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rand/xorshift.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/rand/xorshift.rs
layout: document
redirect_from:
- /library/src/rand/xorshift.rs
- /library/src/rand/xorshift.rs.html
title: src/rand/xorshift.rs
---
