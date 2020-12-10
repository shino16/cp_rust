---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::rand::seed::*;\n\npub struct Xoshiro256plus([u64; 4]);\n\nimpl\
    \ Xoshiro256plus {\n\tpub fn new() -> Self {\n\t\tSelf(seed())\n\t}\n\tpub fn\
    \ next(&mut self) -> u64 {\n\t\tlet s = &mut self.0;\n\t\tlet t = s[1] << 17;\n\
    \t\ts[2] ^= s[0];\n\t\ts[3] ^= s[1];\n\t\ts[1] ^= s[2];\n\t\ts[0] ^= s[3];\n\t\
    \ts[2] ^= t;\n\t\ts[3] = s[3].rotate_left(45);\n\t\ts[0].wrapping_add(s[3])\n\t\
    }\n\t// forward by 2^128\n\tpub fn split(&mut self) -> Self {\n\t\tstatic JUMP:\
    \ [u64; 4] =\n\t\t\t[0x180ec6d33cfd0aba, 0xd5a61266f0c9392c, 0xa9582618e03fc9aa,\
    \ 0x39abdc4529b1661c];\n\t\tlet mut s2 = [0; 4];\n\t\tfor &jump in &JUMP {\n\t\
    \t\tfor b in 0..64 {\n\t\t\t\tif (jump >> b) & 1 != 0 {\n\t\t\t\t\tfor (s2, s)\
    \ in s2.iter_mut().zip(&self.0) {\n\t\t\t\t\t\t*s2 ^= s;\n\t\t\t\t\t}\n\t\t\t\t\
    }\n\t\t\t\tself.next();\n\t\t\t}\n\t\t}\n\t\tSelf(s2)\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rand/xoshiro256plus.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/rand/xoshiro256plus.rs
layout: document
redirect_from:
- /library/src/rand/xoshiro256plus.rs
- /library/src/rand/xoshiro256plus.rs.html
title: src/rand/xoshiro256plus.rs
---
