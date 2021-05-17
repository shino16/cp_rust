---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/rand/xorshift.rs
    title: src/rand/xorshift.rs
  - icon: ':warning:'
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/rand/seed.rs\n"
  code: "#[cfg(unix)]\nmod detail {\n    extern \"system\" {\n        #[link_name\
    \ = \"getrandom\"]\n        fn getrandom(buf: *mut u8, len: usize, flags: usize)\
    \ -> isize;\n    }\n    pub(super) fn fill_bytes(buf: *mut u8, len: usize) {\n\
    \        let ret = unsafe { getrandom(buf, len, 0) };\n        assert_eq!(ret,\
    \ len as isize);\n    }\n}\n\n#[cfg(windows)]\nmod detail {\n    extern \"system\"\
    \ {\n        #[link_name = \"SystemFunction036\"]\n        fn RtlGenRandom(buf:\
    \ *mut u8, len: u32) -> u8;\n    }\n    pub(super) fn fill_bytes(buf: *mut u8,\
    \ len: usize) {\n        let ret = unsafe { RtlGenRandom(buf, len as u32) };\n\
    \        assert_ne!(ret, 0);\n    }\n}\n\nmacro_rules! def_seed {\n    ($(pub\
    \ fn $name:ident() -> $ret:ty;)*) => { $(\n        pub fn $name() -> $ret {\n\
    \            let mut v = std::mem::MaybeUninit::uninit();\n            self::detail::fill_bytes(v.as_mut_ptr()\
    \ as *mut u8, std::mem::size_of_val(&v));\n            unsafe { v.assume_init()\
    \ }\n        }\n    )* };\n}\n\ndef_seed! {\n    pub fn seed() -> [u64; 4];\n\
    \    pub fn seed64() -> u64;\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rand/seed.rs
  requiredBy:
  - src/rand/xoshiro256plus.rs
  - src/rand/xorshift.rs
  - src/tests.rs
  timestamp: '2021-05-17 15:14:26+09:00'
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
