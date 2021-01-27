---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  _extendedRequiredBy:
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/rand/xoshiro256plus.rs\n"
  code: "use crate::rand::seed::*;\n\npub struct Xoshiro256plus([u64; 4]);\n\nimpl\
    \ Xoshiro256plus {\n\tpub fn new() -> Self {\n\t\tSelf(seed())\n\t}\n\tpub fn\
    \ next(&mut self) -> u64 {\n\t\tlet s = &mut self.0;\n\t\tlet t = s[1] << 17;\n\
    \t\ts[2] ^= s[0];\n\t\ts[3] ^= s[1];\n\t\ts[1] ^= s[2];\n\t\ts[0] ^= s[3];\n\t\
    \ts[2] ^= t;\n\t\ts[3] = s[3].rotate_left(45);\n\t\ts[0].wrapping_add(s[3])\n\t\
    }\n\t/// skip 2^128 steps\n\tpub fn split(&mut self) -> Self {\n\t\tstatic JUMP:\
    \ [u64; 4] =\n\t\t\t[0x180ec6d33cfd0aba, 0xd5a61266f0c9392c, 0xa9582618e03fc9aa,\
    \ 0x39abdc4529b1661c];\n\t\tlet mut s2 = [0; 4];\n\t\tfor &jump in &JUMP {\n\t\
    \t\tfor b in 0..64 {\n\t\t\t\tif (jump >> b) & 1 != 0 {\n\t\t\t\t\tfor (s2, s)\
    \ in s2.iter_mut().zip(&self.0) {\n\t\t\t\t\t\t*s2 ^= s;\n\t\t\t\t\t}\n\t\t\t\t\
    }\n\t\t\t\tself.next();\n\t\t\t}\n\t\t}\n\t\tSelf(s2)\n\t}\n}\n"
  dependsOn:
  - src/rand/seed.rs
  isVerificationFile: false
  path: src/rand/xoshiro256plus.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-01-07 16:16:30+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/rand/xoshiro256plus.rs
layout: document
redirect_from:
- /library/src/rand/xoshiro256plus.rs
- /library/src/rand/xoshiro256plus.rs.html
title: src/rand/xoshiro256plus.rs
---
