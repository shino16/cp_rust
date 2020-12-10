---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub struct Xorshift32(u32);\n\nimpl Xorshift32 {\n\tpub fn new() -> Self\
    \ {\n\t\tSelf(2_463_534_242)\n\t}\n\tpub fn next(&mut self) -> u32 {\n\t\tlet\
    \ mut x = self.0;\n\t\tx ^= x << 13;\n\t\tx ^= x >> 17;\n\t\tx ^= x << 5;\n\t\t\
    self.0 = x;\n\t\tx\n\t}\n}\n\npub struct Xorshift64(u64);\n\nimpl Xorshift64 {\n\
    \tpub fn new() -> Self {\n\t\tSelf(88_172_645_463_325_252)\n\t}\n\tpub fn next(&mut\
    \ self) -> u64 {\n\t\tlet mut x = self.0;\n\t\tx ^= x << 13;\n\t\tx ^= x >> 7;\n\
    \t\tx ^= x << 17;\n\t\tself.0 = x;\n\t\tx\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rand/xorshift.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/rand/xorshift.rs
layout: document
redirect_from:
- /library/src/rand/xorshift.rs
- /library/src/rand/xorshift.rs.html
title: src/rand/xorshift.rs
---
