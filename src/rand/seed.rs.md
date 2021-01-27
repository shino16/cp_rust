---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':heavy_check_mark:'
    path: src/fxhash.rs
    title: src/fxhash.rs
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
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/rand/seed.rs\n"
  code: "#[cfg(windows)]\nmod imp {\n\textern \"system\" {\n\t\t#[link_name = \"SystemFunction036\"\
    ]\n\t\tfn RtlGenRandom(buf: *mut u8, len: u32) -> u8;\n\t}\n\tfn getrand(buf:\
    \ *mut u8, len: usize) {\n\t\tlet ret = unsafe { RtlGenRandom(buf, len as u32)\
    \ };\n\t\tassert_ne!(ret, 0);\n\t}\n\tmacro_rules! def_seed {\n\t\t($(pub fn $name:ident()\
    \ -> $ret:ty;)*) => { $(\n\t\t\tpub fn $name() -> $ret {\n\t\t\t\tlet mut v =\
    \ std::mem::MaybeUninit::uninit();\n\t\t\t\tgetrand(v.as_mut_ptr() as *mut u8,\
    \ std::mem::size_of_val(&v));\n\t\t\t\tunsafe { v.assume_init() }\n\t\t\t}\n\t\
    \t)* };\n\t}\n\tdef_seed! {\n\t\tpub fn seed() -> [u64; 4];\n\t\tpub fn seed64()\
    \ -> u64;\n\t}\n}\n\n#[cfg(not(windows))]\nmod imp {\n\tpub fn seed() -> [u64;\
    \ 4] {\n\t\t[\n\t\t\t// arbitrary\n\t\t\t0x35fee63b_fd9f69cf,\n\t\t\t0x9fd0680a_f9e37356,\n\
    \t\t\t0x7454d5e3_d982527e,\n\t\t\t0x35d1849a_77925163,\n\t\t]\n\t}\n\tpub fn seed64()\
    \ -> u64 {\n\t\t0x17adfb20_0995921c\n\t}\n}\n\npub use self::imp::*; // Edition\
    \ 2015 support\n"
  dependsOn: []
  isVerificationFile: false
  path: src/rand/seed.rs
  requiredBy:
  - src/tests.rs
  - src/dfa.rs
  - src/fxhash.rs
  - src/rand/xoshiro256plus.rs
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/cargo_test.rs
documentation_of: src/rand/seed.rs
layout: document
redirect_from:
- /library/src/rand/seed.rs
- /library/src/rand/seed.rs.html
title: src/rand/seed.rs
---
