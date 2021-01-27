---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  _extendedRequiredBy:
  - icon: ':x:'
    path: src/dfa.rs
    title: src/dfa.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/fxhash.rs\n"
  code: "// Copyright 2015 The Rust Project Developers. See the COPYRIGHT\n// file\
    \ at the top-level directory of this distribution and at\n// http://rust-lang.org/COPYRIGHT.\n\
    //\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n// http://www.apache.org/licenses/LICENSE-2.0>\
    \ or the MIT license\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>,\
    \ at your\n// option. This file may not be copied, modified, or distributed\n\
    // except according to those terms.\n\nuse crate::rand::seed;\nuse std::collections::{HashMap,\
    \ HashSet};\nuse std::default::Default;\nuse std::hash::{BuildHasherDefault, Hash,\
    \ Hasher};\nuse std::ops::BitXor;\n\npub type FxBuildHasher = BuildHasherDefault<FxHasher>;\n\
    \npub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;\n\npub type FxHashSet<V>\
    \ = HashSet<V, FxBuildHasher>;\n\nconst ROTATE: u32 = 5;\nstatic mut SEED64: u64\
    \ = 0;\nstatic mut SEED32: u32 = 0;\nstatic mut SEED: usize = 0;\n\n#[used]\n\
    #[cfg_attr(target_os = \"linux\", link_section = \".init_array\")]\n#[cfg_attr(target_os\
    \ = \"windows\", link_section = \".CRT$XCU\")]\nstatic INIT: unsafe extern \"\
    C\" fn() = {\n\t#[cfg_attr(target_os = \"linux\", link_section = \".text.startup\"\
    )]\n\tunsafe extern \"C\" fn init() {\n\t\tSEED64 = seed::seed64();\n\t\tSEED32\
    \ = SEED64 as u32;\n\t\tSEED = SEED64 as usize;\n\t}\n\tinit\n};\n\ntrait HashWord\
    \ {\n\tfn hash_word(&mut self, word: Self);\n}\n\nmacro_rules! impl_hash_word\
    \ {\n\t($($ty:ty = $key:ident),* $(,)*) => { $(\n\t\timpl HashWord for $ty {\n\
    \t\t\tfn hash_word(&mut self, word: Self) {\n\t\t\t\t*self = self.rotate_left(ROTATE).bitxor(word).wrapping_mul(unsafe\
    \ { $key });\n\t\t\t}\n\t\t}\n\t)* }\n}\n\nimpl_hash_word!(usize = SEED, u32 =\
    \ SEED32, u64 = SEED64);\n\nfn read_u32(bytes: &[u8]) -> u32 {\n\tlet mut data\
    \ = 0;\n\tunsafe {\n\t\tstd::ptr::copy_nonoverlapping(bytes.as_ptr(), &mut data\
    \ as *mut _ as *mut u8, 4);\n\t}\n\tdata\n}\n\nfn read_u64(bytes: &[u8]) -> u64\
    \ {\n\tlet mut data = 0;\n\tunsafe {\n\t\tstd::ptr::copy_nonoverlapping(bytes.as_ptr(),\
    \ &mut data as *mut _ as *mut u8, 8);\n\t}\n\tdata\n}\n\n#[allow(dead_code)]\n\
    fn write32(mut hash: u32, mut bytes: &[u8]) -> u32 {\n\twhile bytes.len() >= 4\
    \ {\n\t\tlet n = read_u32(bytes);\n\t\thash.hash_word(n);\n\t\tbytes = bytes.split_at(4).1;\n\
    \t}\n\n\tfor byte in bytes {\n\t\thash.hash_word(*byte as u32);\n\t}\n\thash\n\
    }\n\n#[allow(dead_code)]\nfn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {\n\
    \twhile bytes.len() >= 8 {\n\t\tlet n = read_u64(bytes);\n\t\thash.hash_word(n);\n\
    \t\tbytes = bytes.split_at(8).1;\n\t}\n\n\tif bytes.len() >= 4 {\n\t\tlet n =\
    \ read_u32(bytes);\n\t\thash.hash_word(n as u64);\n\t\tbytes = bytes.split_at(4).1;\n\
    \t}\n\n\tfor byte in bytes {\n\t\thash.hash_word(*byte as u64);\n\t}\n\thash\n\
    }\n\n#[cfg(target_pointer_width = \"32\")]\nfn write(hash: usize, bytes: &[u8])\
    \ -> usize {\n\twrite32(hash as u32, bytes) as usize\n}\n\n#[cfg(target_pointer_width\
    \ = \"64\")]\nfn write(hash: usize, bytes: &[u8]) -> usize {\n\twrite64(hash as\
    \ u64, bytes) as usize\n}\n\n#[derive(Debug, Clone)]\npub struct FxHasher {\n\t\
    hash: usize,\n}\n\nimpl Default for FxHasher {\n\tfn default() -> FxHasher {\n\
    \t\tFxHasher { hash: 0 }\n\t}\n}\n\nimpl Hasher for FxHasher {\n\tfn write(&mut\
    \ self, bytes: &[u8]) {\n\t\tself.hash = write(self.hash, bytes);\n\t}\n\n\tfn\
    \ write_u8(&mut self, i: u8) {\n\t\tself.hash.hash_word(i as usize);\n\t}\n\n\t\
    fn write_u16(&mut self, i: u16) {\n\t\tself.hash.hash_word(i as usize);\n\t}\n\
    \n\tfn write_u32(&mut self, i: u32) {\n\t\tself.hash.hash_word(i as usize);\n\t\
    }\n\n\t#[cfg(target_pointer_width = \"32\")]\n\tfn write_u64(&mut self, i: u64)\
    \ {\n\t\tself.hash.hash_word(i as usize);\n\t\tself.hash.hash_word((i >> 32) as\
    \ usize);\n\t}\n\n\t#[cfg(target_pointer_width = \"64\")]\n\tfn write_u64(&mut\
    \ self, i: u64) {\n\t\tself.hash.hash_word(i as usize);\n\t}\n\n\tfn write_usize(&mut\
    \ self, i: usize) {\n\t\tself.hash.hash_word(i);\n\t}\n\n\tfn finish(&self) ->\
    \ u64 {\n\t\tself.hash as u64\n\t}\n}\n\npub fn hash<T: Hash + ?Sized>(v: &T)\
    \ -> usize {\n\tlet mut state = FxHasher::default();\n\tv.hash(&mut state);\n\t\
    state.finish() as usize\n}\n"
  dependsOn:
  - src/rand/seed.rs
  isVerificationFile: false
  path: src/fxhash.rs
  requiredBy:
  - src/dfa.rs
  timestamp: '2021-01-03 22:19:51+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
documentation_of: src/fxhash.rs
layout: document
redirect_from:
- /library/src/fxhash.rs
- /library/src/fxhash.rs.html
title: src/fxhash.rs
---
