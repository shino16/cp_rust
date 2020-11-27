---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub struct Rng32(u32);\n\nimpl Rng32 {\n\tpub fn new() -> Self {\n\t\tRng32(2_463_534_242)\n\
    \t}\n\tpub fn gen(&mut self) -> u32 {\n\t\tlet mut x = self.0;\n\t\tx ^= x <<\
    \ 13;\n\t\tx ^= x >> 17;\n\t\tx ^= x << 5;\n\t\tself.0 = x;\n\t\tx\n\t}\n}\n\n\
    pub struct Rng64(u64);\n\nimpl Rng64 {\n\tpub fn new() -> Self {\n\t\tRng64(88_172_645_463_325_252)\n\
    \t}\n\tpub fn gen(&mut self) -> u64 {\n\t\tlet mut x = self.0;\n\t\tx ^= x <<\
    \ 13;\n\t\tx ^= x >> 7;\n\t\tx ^= x << 17;\n\t\tself.0 = x;\n\t\tx\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rng.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/rng.rs
layout: document
redirect_from:
- /library/src/rng.rs
- /library/src/rng.rs.html
title: src/rng.rs
---
