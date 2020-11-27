---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// Copyright 2015 The Rust Project Developers. See the COPYRIGHT\n// file\
    \ at the top-level directory of this distribution and at\n// http://rust-lang.org/COPYRIGHT.\n\
    //\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n// http://www.apache.org/licenses/LICENSE-2.0>\
    \ or the MIT license\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>,\
    \ at your\n// option. This file may not be copied, modified, or distributed\n\
    // except according to those terms.\n\nuse std::collections::{HashMap, HashSet};\n\
    use std::default::Default;\nuse std::hash::{BuildHasherDefault, Hash, Hasher};\n\
    use std::ops::BitXor;\n\npub type FxBuildHasher = BuildHasherDefault<FxHasher>;\n\
    \npub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;\n\npub type FxHashSet<V>\
    \ = HashSet<V, FxBuildHasher>;\n\nconst ROTATE: u32 = 5;\nconst SEED64: u64 =\
    \ 0x517cc1b727220a95;\nconst SEED32: u32 = (SEED64 & 0xFFFF_FFFF) as u32;\n\n\
    #[cfg(target_pointer_width = \"32\")]\nconst SEED: usize = SEED32 as usize;\n\
    #[cfg(target_pointer_width = \"64\")]\nconst SEED: usize = SEED64 as usize;\n\n\
    trait HashWord {\n\tfn hash_word(&mut self, word: Self);\n}\n\nmacro_rules! impl_hash_word\
    \ {\n\t($($ty:ty = $key:ident),* $(,)*) => { $(\n\t\timpl HashWord for $ty {\n\
    \t\t\t#[inline]\n\t\t\tfn hash_word(&mut self, word: Self) {\n\t\t\t\t*self =\
    \ self.rotate_left(ROTATE).bitxor(word).wrapping_mul($key);\n\t\t\t}\n\t\t}\n\t\
    )* }\n}\n\nimpl_hash_word!(usize = SEED, u32 = SEED32, u64 = SEED64);\n\nfn read_u32(bytes:\
    \ &[u8]) -> u32 {\n\tlet mut data = 0;\n\tunsafe {\n\t\tstd::ptr::copy_nonoverlapping(bytes.as_ptr(),\
    \ &mut data as *mut _ as *mut u8, 4);\n\t}\n\tdata\n}\n\nfn read_u64(bytes: &[u8])\
    \ -> u64 {\n\tlet mut data = 0;\n\tunsafe {\n\t\tstd::ptr::copy_nonoverlapping(bytes.as_ptr(),\
    \ &mut data as *mut _ as *mut u8, 8);\n\t}\n\tdata\n}\n\n#[inline]\nfn write32(mut\
    \ hash: u32, mut bytes: &[u8]) -> u32 {\n\twhile bytes.len() >= 4 {\n\t\tlet n\
    \ = read_u32(bytes);\n\t\thash.hash_word(n);\n\t\tbytes = bytes.split_at(4).1;\n\
    \t}\n\n\tfor byte in bytes {\n\t\thash.hash_word(*byte as u32);\n\t}\n\thash\n\
    }\n\n#[inline]\nfn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {\n\twhile\
    \ bytes.len() >= 8 {\n\t\tlet n = read_u64(bytes);\n\t\thash.hash_word(n);\n\t\
    \tbytes = bytes.split_at(8).1;\n\t}\n\n\tif bytes.len() >= 4 {\n\t\tlet n = read_u32(bytes);\n\
    \t\thash.hash_word(n as u64);\n\t\tbytes = bytes.split_at(4).1;\n\t}\n\n\tfor\
    \ byte in bytes {\n\t\thash.hash_word(*byte as u64);\n\t}\n\thash\n}\n\n#[inline]\n\
    #[cfg(target_pointer_width = \"32\")]\nfn write(hash: usize, bytes: &[u8]) ->\
    \ usize { write32(hash as u32, bytes) as usize }\n\n#[inline]\n#[cfg(target_pointer_width\
    \ = \"64\")]\nfn write(hash: usize, bytes: &[u8]) -> usize { write64(hash as u64,\
    \ bytes) as usize }\n\n#[derive(Debug, Clone)]\npub struct FxHasher {\n\thash:\
    \ usize,\n}\n\nimpl Default for FxHasher {\n\t#[inline]\n\tfn default() -> FxHasher\
    \ { FxHasher { hash: 0 } }\n}\n\nimpl Hasher for FxHasher {\n\t#[inline]\n\tfn\
    \ write(&mut self, bytes: &[u8]) { self.hash = write(self.hash, bytes); }\n\n\t\
    #[inline]\n\tfn write_u8(&mut self, i: u8) { self.hash.hash_word(i as usize);\
    \ }\n\n\t#[inline]\n\tfn write_u16(&mut self, i: u16) { self.hash.hash_word(i\
    \ as usize); }\n\n\t#[inline]\n\tfn write_u32(&mut self, i: u32) { self.hash.hash_word(i\
    \ as usize); }\n\n\t#[inline]\n\t#[cfg(target_pointer_width = \"32\")]\n\tfn write_u64(&mut\
    \ self, i: u64) {\n\t\tself.hash.hash_word(i as usize);\n\t\tself.hash.hash_word((i\
    \ >> 32) as usize);\n\t}\n\n\t#[inline]\n\t#[cfg(target_pointer_width = \"64\"\
    )]\n\tfn write_u64(&mut self, i: u64) { self.hash.hash_word(i as usize); }\n\n\
    \t#[inline]\n\tfn write_usize(&mut self, i: usize) { self.hash.hash_word(i); }\n\
    \n\t#[inline]\n\tfn finish(&self) -> u64 { self.hash as u64 }\n}\n\n#[derive(Debug,\
    \ Clone)]\npub struct FxHasher64 {\n\thash: u64,\n}\n\nimpl Default for FxHasher64\
    \ {\n\t#[inline]\n\tfn default() -> FxHasher64 { FxHasher64 { hash: 0 } }\n}\n\
    \nimpl Hasher for FxHasher64 {\n\t#[inline]\n\tfn write(&mut self, bytes: &[u8])\
    \ { self.hash = write64(self.hash, bytes); }\n\n\t#[inline]\n\tfn write_u8(&mut\
    \ self, i: u8) { self.hash.hash_word(i as u64); }\n\n\t#[inline]\n\tfn write_u16(&mut\
    \ self, i: u16) { self.hash.hash_word(i as u64); }\n\n\t#[inline]\n\tfn write_u32(&mut\
    \ self, i: u32) { self.hash.hash_word(i as u64); }\n\n\tfn write_u64(&mut self,\
    \ i: u64) { self.hash.hash_word(i); }\n\n\t#[inline]\n\tfn write_usize(&mut self,\
    \ i: usize) { self.hash.hash_word(i as u64); }\n\n\t#[inline]\n\tfn finish(&self)\
    \ -> u64 { self.hash }\n}\n\n#[derive(Debug, Clone)]\npub struct FxHasher32 {\n\
    \thash: u32,\n}\n\nimpl Default for FxHasher32 {\n\t#[inline]\n\tfn default()\
    \ -> FxHasher32 { FxHasher32 { hash: 0 } }\n}\n\nimpl Hasher for FxHasher32 {\n\
    \t#[inline]\n\tfn write(&mut self, bytes: &[u8]) { self.hash = write32(self.hash,\
    \ bytes); }\n\n\t#[inline]\n\tfn write_u8(&mut self, i: u8) { self.hash.hash_word(i\
    \ as u32); }\n\n\t#[inline]\n\tfn write_u16(&mut self, i: u16) { self.hash.hash_word(i\
    \ as u32); }\n\n\t#[inline]\n\tfn write_u32(&mut self, i: u32) { self.hash.hash_word(i);\
    \ }\n\n\t#[inline]\n\tfn write_u64(&mut self, i: u64) {\n\t\tself.hash.hash_word(i\
    \ as u32);\n\t\tself.hash.hash_word((i >> 32) as u32);\n\t}\n\n\t#[inline]\n\t\
    #[cfg(target_pointer_width = \"32\")]\n\tfn write_usize(&mut self, i: usize) {\
    \ self.write_u32(i as u32); }\n\n\t#[inline]\n\t#[cfg(target_pointer_width = \"\
    64\")]\n\tfn write_usize(&mut self, i: usize) { self.write_u64(i as u64); }\n\n\
    \t#[inline]\n\tfn finish(&self) -> u64 { self.hash as u64 }\n}\n\n#[inline]\n\
    pub fn hash64<T: Hash + ?Sized>(v: &T) -> u64 {\n\tlet mut state = FxHasher64::default();\n\
    \tv.hash(&mut state);\n\tstate.finish()\n}\n\n#[inline]\npub fn hash32<T: Hash\
    \ + ?Sized>(v: &T) -> u32 {\n\tlet mut state = FxHasher32::default();\n\tv.hash(&mut\
    \ state);\n\tstate.finish() as u32\n}\n\n#[inline]\npub fn hash<T: Hash + ?Sized>(v:\
    \ &T) -> usize {\n\tlet mut state = FxHasher::default();\n\tv.hash(&mut state);\n\
    \tstate.finish() as usize\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/fxhash.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
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
