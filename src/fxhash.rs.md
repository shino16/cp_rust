---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  _extendedVerifiedWith:
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
    C\" fn() = {\n    #[cfg_attr(target_os = \"linux\", link_section = \".text.startup\"\
    )]\n    unsafe extern \"C\" fn init() {\n        SEED64 = seed::seed64();\n  \
    \      SEED32 = SEED64 as u32;\n        SEED = SEED64 as usize;\n    }\n    init\n\
    };\n\ntrait HashWord {\n    fn hash_word(&mut self, word: Self);\n}\n\nmacro_rules!\
    \ impl_hash_word {\n    ($($ty:ty = $key:ident),* $(,)*) => { $(\n        impl\
    \ HashWord for $ty {\n            fn hash_word(&mut self, word: Self) {\n    \
    \            *self = self.rotate_left(ROTATE).bitxor(word).wrapping_mul(unsafe\
    \ { $key });\n            }\n        }\n    )* }\n}\n\nimpl_hash_word!(usize =\
    \ SEED, u32 = SEED32, u64 = SEED64);\n\nfn read_u32(bytes: &[u8]) -> u32 {\n \
    \   let mut data = 0;\n    unsafe {\n        std::ptr::copy_nonoverlapping(bytes.as_ptr(),\
    \ &mut data as *mut _ as *mut u8, 4);\n    }\n    data\n}\n\nfn read_u64(bytes:\
    \ &[u8]) -> u64 {\n    let mut data = 0;\n    unsafe {\n        std::ptr::copy_nonoverlapping(bytes.as_ptr(),\
    \ &mut data as *mut _ as *mut u8, 8);\n    }\n    data\n}\n\n#[allow(dead_code)]\n\
    fn write32(mut hash: u32, mut bytes: &[u8]) -> u32 {\n    while bytes.len() >=\
    \ 4 {\n        let n = read_u32(bytes);\n        hash.hash_word(n);\n        bytes\
    \ = bytes.split_at(4).1;\n    }\n\n    for byte in bytes {\n        hash.hash_word(*byte\
    \ as u32);\n    }\n    hash\n}\n\n#[allow(dead_code)]\nfn write64(mut hash: u64,\
    \ mut bytes: &[u8]) -> u64 {\n    while bytes.len() >= 8 {\n        let n = read_u64(bytes);\n\
    \        hash.hash_word(n);\n        bytes = bytes.split_at(8).1;\n    }\n\n \
    \   if bytes.len() >= 4 {\n        let n = read_u32(bytes);\n        hash.hash_word(n\
    \ as u64);\n        bytes = bytes.split_at(4).1;\n    }\n\n    for byte in bytes\
    \ {\n        hash.hash_word(*byte as u64);\n    }\n    hash\n}\n\n#[cfg(target_pointer_width\
    \ = \"32\")]\nfn write(hash: usize, bytes: &[u8]) -> usize {\n    write32(hash\
    \ as u32, bytes) as usize\n}\n\n#[cfg(target_pointer_width = \"64\")]\nfn write(hash:\
    \ usize, bytes: &[u8]) -> usize {\n    write64(hash as u64, bytes) as usize\n\
    }\n\n#[derive(Debug, Clone)]\npub struct FxHasher {\n    hash: usize,\n}\n\nimpl\
    \ Default for FxHasher {\n    fn default() -> FxHasher {\n        FxHasher { hash:\
    \ 0 }\n    }\n}\n\nimpl Hasher for FxHasher {\n    fn write(&mut self, bytes:\
    \ &[u8]) {\n        self.hash = write(self.hash, bytes);\n    }\n\n    fn write_u8(&mut\
    \ self, i: u8) {\n        self.hash.hash_word(i as usize);\n    }\n\n    fn write_u16(&mut\
    \ self, i: u16) {\n        self.hash.hash_word(i as usize);\n    }\n\n    fn write_u32(&mut\
    \ self, i: u32) {\n        self.hash.hash_word(i as usize);\n    }\n\n    #[cfg(target_pointer_width\
    \ = \"32\")]\n    fn write_u64(&mut self, i: u64) {\n        self.hash.hash_word(i\
    \ as usize);\n        self.hash.hash_word((i >> 32) as usize);\n    }\n\n    #[cfg(target_pointer_width\
    \ = \"64\")]\n    fn write_u64(&mut self, i: u64) {\n        self.hash.hash_word(i\
    \ as usize);\n    }\n\n    fn write_usize(&mut self, i: usize) {\n        self.hash.hash_word(i);\n\
    \    }\n\n    fn finish(&self) -> u64 {\n        self.hash as u64\n    }\n}\n\n\
    pub fn hash<T: Hash + ?Sized>(v: &T) -> usize {\n    let mut state = FxHasher::default();\n\
    \    v.hash(&mut state);\n    state.finish() as usize\n}\n"
  dependsOn:
  - src/rand/seed.rs
  isVerificationFile: false
  path: src/fxhash.rs
  requiredBy:
  - src/dfa.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/dfa_test.rs
documentation_of: src/fxhash.rs
layout: document
redirect_from:
- /library/src/fxhash.rs
- /library/src/fxhash.rs.html
title: src/fxhash.rs
---
