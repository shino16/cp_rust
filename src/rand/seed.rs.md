---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/rand/xorshift.rs
    title: src/rand/xorshift.rs
  - icon: ':heavy_check_mark:'
    path: src/rand/xoshiro256plus.rs
    title: src/rand/xoshiro256plus.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/rand/seed.rs\n"
  code: "// Codeforces\n#[cfg(windows)]\nmod detail {\n    extern \"system\" {\n \
    \       #[link_name = \"SystemFunction036\"]\n        fn RtlGenRandom(buf: *mut\
    \ u8, len: u32) -> u8;\n    }\n    fn getrand(buf: *mut u8, len: usize) {\n  \
    \      let ret = unsafe { RtlGenRandom(buf, len as u32) };\n        assert_ne!(ret,\
    \ 0);\n    }\n    macro_rules! def_seed {\n        ($(pub fn $name:ident() ->\
    \ $ret:ty;)*) => { $(\n            pub fn $name() -> $ret {\n                let\
    \ mut v = std::mem::MaybeUninit::uninit();\n                getrand(v.as_mut_ptr()\
    \ as *mut u8, std::mem::size_of_val(&v));\n                unsafe { v.assume_init()\
    \ }\n            }\n        )* };\n    }\n    def_seed! {\n        pub fn seed()\
    \ -> [u64; 4];\n        pub fn seed64() -> u64;\n    }\n}\n\n#[cfg(not(windows))]\n\
    mod detail {\n    pub fn seed() -> [u64; 4] {\n        [\n            0x35fee63b_fd9f69cf,\n\
    \            0x9fd0680a_f9e37356,\n            0x7454d5e3_d982527e,\n        \
    \    0x35d1849a_77925163,\n        ]\n    }\n    pub fn seed64() -> u64 { 0x17adfb20_0995921c\
    \ }\n}\n\npub use self::detail::*;\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rand/seed.rs
  requiredBy:
  - src/rand/xoshiro256plus.rs
  - src/rand/xorshift.rs
  - src/tests.rs
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/rand/seed.rs
layout: document
redirect_from:
- /library/src/rand/seed.rs
- /library/src/rand/seed.rs.html
title: src/rand/seed.rs
---
