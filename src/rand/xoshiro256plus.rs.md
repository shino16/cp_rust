---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/rand/xoshiro256plus.rs\n"
  code: "use crate::rand::seed::*;\n\npub struct Xoshiro256plus([u64; 4]);\n\nimpl\
    \ Xoshiro256plus {\n    pub fn new() -> Self {\n        Self(from_time())\n  \
    \  }\n    pub fn next(&mut self) -> u64 {\n        let s = &mut self.0;\n    \
    \    let t = s[1] << 17;\n        s[2] ^= s[0];\n        s[3] ^= s[1];\n     \
    \   s[1] ^= s[2];\n        s[0] ^= s[3];\n        s[2] ^= t;\n        s[3] = s[3].rotate_left(45);\n\
    \        s[0].wrapping_add(s[3])\n    }\n    /// skip 2^128 steps\n    pub fn\
    \ split(&mut self) -> Self {\n        static JUMP: [u64; 4] =\n            [0x180ec6d33cfd0aba,\
    \ 0xd5a61266f0c9392c, 0xa9582618e03fc9aa, 0x39abdc4529b1661c];\n        let mut\
    \ s2 = [0; 4];\n        for &jump in &JUMP {\n            for b in 0..64 {\n \
    \               if (jump >> b) & 1 != 0 {\n                    for (s2, s) in\
    \ s2.iter_mut().zip(&self.0) {\n                        *s2 ^= s;\n          \
    \          }\n                }\n                self.next();\n            }\n\
    \        }\n        Self(s2)\n    }\n}\n"
  dependsOn:
  - src/rand/seed.rs
  isVerificationFile: false
  path: src/rand/xoshiro256plus.rs
  requiredBy:
  - src/tests.rs
  timestamp: '2021-02-22 02:21:06+09:00'
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
