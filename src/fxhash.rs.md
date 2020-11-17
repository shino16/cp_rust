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
    // except according to those terms.\n\n//! # Fx Hash\n//!\n//! This hashing algorithm\
    \ was extracted from the Rustc compiler.  This is the same hashing\n//! algoirthm\
    \ used for some internal operations in FireFox.  The strength of this algorithm\n\
    //! is in hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm\
    \ works on one\n//! byte at a time.\n//!\n//! ## Disclaimer\n//!\n//! It is **not\
    \ a cryptographically secure** hash, so it is strongly recommended that you do\n\
    //! not use this hash for cryptographic purproses.  Furthermore, this hashing\
    \ algorithm was\n//! not designed to prevent any attacks for determining collisions\
    \ which could be used to\n//! potentially cause quadratic behavior in `HashMap`s.\
    \  So it is not recommended to expose\n//! this hash in places where collissions\
    \ or DDOS attacks may be a concern.\n\nuse std::collections::{HashMap, HashSet};\n\
    use std::default::Default;\nuse std::hash::{Hasher, Hash, BuildHasherDefault};\n\
    use std::ops::BitXor;\n\n/// A builder for default Fx hashers.\npub type FxBuildHasher\
    \ = BuildHasherDefault<FxHasher>;\n\n/// A `HashMap` using a default Fx hasher.\n\
    pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;\n\n/// A `HashSet` using\
    \ a default Fx hasher.\npub type FxHashSet<V> = HashSet<V, FxBuildHasher>;\n\n\
    const ROTATE: u32 = 5;\nconst SEED64: u64 = 0x517cc1b727220a95;\nconst SEED32:\
    \ u32 = (SEED64 & 0xFFFF_FFFF) as u32;\n\n#[cfg(target_pointer_width = \"32\"\
    )]\nconst SEED: usize = SEED32 as usize;\n#[cfg(target_pointer_width = \"64\"\
    )]\nconst SEED: usize = SEED64 as usize;\n\ntrait HashWord {\n    fn hash_word(&mut\
    \ self, word: Self);\n}\n\nmacro_rules! impl_hash_word {\n    ($($ty:ty = $key:ident),*\
    \ $(,)*) => (\n        $(\n            impl HashWord for $ty {\n             \
    \   #[inline]\n                fn hash_word(&mut self, word: Self) {\n       \
    \             *self = self.rotate_left(ROTATE).bitxor(word).wrapping_mul($key);\n\
    \                }\n            }\n        )*\n    )\n}\n\nimpl_hash_word!(usize\
    \ = SEED, u32 = SEED32, u64 = SEED64);\n\nfn read_u32(bytes: &[u8]) -> u32 {\n\
    \    let mut data = 0;\n    unsafe {\n        std::ptr::copy_nonoverlapping(\n\
    \            bytes.as_ptr(),\n            &mut data as *mut _ as *mut u8,\n  \
    \          4);\n    }\n    data\n}\n\nfn read_u64(bytes: &[u8]) -> u64 {\n   \
    \ let mut data = 0;\n    unsafe {\n        std::ptr::copy_nonoverlapping(\n  \
    \          bytes.as_ptr(),\n            &mut data as *mut _ as *mut u8,\n    \
    \        8);\n    }\n    data\n}\n\n#[inline]\nfn write32(mut hash: u32, mut bytes:\
    \ &[u8]) -> u32 {\n    while bytes.len() >= 4 {\n        let n = read_u32(bytes);\n\
    \        hash.hash_word(n);\n        bytes = bytes.split_at(4).1;\n    }\n\n \
    \   for byte in bytes {\n        hash.hash_word(*byte as u32);\n    }\n    hash\n\
    }\n\n#[inline]\nfn write64(mut hash: u64, mut bytes: &[u8]) -> u64 {\n    while\
    \ bytes.len() >= 8 {\n        let n = read_u64(bytes);\n        hash.hash_word(n);\n\
    \        bytes = bytes.split_at(8).1;\n    }\n\n    if bytes.len() >= 4 {\n  \
    \      let n = read_u32(bytes);\n        hash.hash_word(n as u64);\n        bytes\
    \ = bytes.split_at(4).1;\n    }\n\n    for byte in bytes {\n        hash.hash_word(*byte\
    \ as u64);\n    }\n    hash\n}\n\n#[inline]\n#[cfg(target_pointer_width = \"32\"\
    )]\nfn write(hash: usize, bytes: &[u8]) -> usize {\n    write32(hash as u32, bytes)\
    \ as usize\n}\n\n#[inline]\n#[cfg(target_pointer_width = \"64\")]\nfn write(hash:\
    \ usize, bytes: &[u8]) -> usize {\n    write64(hash as u64, bytes) as usize\n\
    }\n\n/// This hashing algorithm was extracted from the Rustc compiler.\n/// This\
    \ is the same hashing algoirthm used for some internal operations in FireFox.\n\
    /// The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms,\n\
    /// where the FNV algorithm works on one byte at a time.\n///\n/// This hashing\
    \ algorithm should not be used for cryptographic, or in scenarios where\n/// DOS\
    \ attacks are a concern.\n#[derive(Debug, Clone)]\npub struct FxHasher {\n   \
    \ hash: usize,\n}\n\nimpl Default for FxHasher {\n    #[inline]\n    fn default()\
    \ -> FxHasher {\n        FxHasher { hash: 0 }\n    }\n}\n\nimpl Hasher for FxHasher\
    \ {\n    #[inline]\n    fn write(&mut self, bytes: &[u8]) {\n        self.hash\
    \ = write(self.hash, bytes);\n    }\n\n    #[inline]\n    fn write_u8(&mut self,\
    \ i: u8) {\n        self.hash.hash_word(i as usize);\n    }\n\n    #[inline]\n\
    \    fn write_u16(&mut self, i: u16) {\n        self.hash.hash_word(i as usize);\n\
    \    }\n\n    #[inline]\n    fn write_u32(&mut self, i: u32) {\n        self.hash.hash_word(i\
    \ as usize);\n    }\n\n    #[inline]\n    #[cfg(target_pointer_width = \"32\"\
    )]\n    fn write_u64(&mut self, i: u64) {\n        self.hash.hash_word(i as usize);\n\
    \        self.hash.hash_word((i >> 32) as usize);\n    }\n\n    #[inline]\n  \
    \  #[cfg(target_pointer_width = \"64\")]\n    fn write_u64(&mut self, i: u64)\
    \ {\n        self.hash.hash_word(i as usize);\n    }\n\n    #[inline]\n    fn\
    \ write_usize(&mut self, i: usize) {\n        self.hash.hash_word(i);\n    }\n\
    \n    #[inline]\n    fn finish(&self) -> u64 {\n        self.hash as u64\n   \
    \ }\n}\n\n/// This hashing algorithm was extracted from the Rustc compiler.\n\
    /// This is the same hashing algoirthm used for some internal operations in FireFox.\n\
    /// The strength of this algorithm is in hashing 8 bytes at a time on any platform,\n\
    /// where the FNV algorithm works on one byte at a time.\n///\n/// This hashing\
    \ algorithm should not be used for cryptographic, or in scenarios where\n/// DOS\
    \ attacks are a concern.\n#[derive(Debug, Clone)]\npub struct FxHasher64 {\n \
    \   hash: u64,\n}\n\nimpl Default for FxHasher64 {\n    #[inline]\n    fn default()\
    \ -> FxHasher64 {\n        FxHasher64 { hash: 0 }\n    }\n}\n\nimpl Hasher for\
    \ FxHasher64 {\n    #[inline]\n    fn write(&mut self, bytes: &[u8]) {\n     \
    \   self.hash = write64(self.hash, bytes);\n    }\n\n    #[inline]\n    fn write_u8(&mut\
    \ self, i: u8) {\n        self.hash.hash_word(i as u64);\n    }\n\n    #[inline]\n\
    \    fn write_u16(&mut self, i: u16) {\n        self.hash.hash_word(i as u64);\n\
    \    }\n\n    #[inline]\n    fn write_u32(&mut self, i: u32) {\n        self.hash.hash_word(i\
    \ as u64);\n    }\n\n    fn write_u64(&mut self, i: u64) {\n        self.hash.hash_word(i);\n\
    \    }\n\n    #[inline]\n    fn write_usize(&mut self, i: usize) {\n        self.hash.hash_word(i\
    \ as u64);\n    }\n\n    #[inline]\n    fn finish(&self) -> u64 {\n        self.hash\n\
    \    }\n}\n\n/// This hashing algorithm was extracted from the Rustc compiler.\n\
    /// This is the same hashing algoirthm used for some internal operations in FireFox.\n\
    /// The strength of this algorithm is in hashing 4 bytes at a time on any platform,\n\
    /// where the FNV algorithm works on one byte at a time.\n///\n/// This hashing\
    \ algorithm should not be used for cryptographic, or in scenarios where\n/// DOS\
    \ attacks are a concern.\n#[derive(Debug, Clone)]\npub struct FxHasher32 {\n \
    \   hash: u32,\n}\n\nimpl Default for FxHasher32 {\n    #[inline]\n    fn default()\
    \ -> FxHasher32 {\n        FxHasher32 { hash: 0 }\n    }\n}\n\nimpl Hasher for\
    \ FxHasher32 {\n    #[inline]\n    fn write(&mut self, bytes: &[u8]) {\n     \
    \   self.hash = write32(self.hash, bytes);\n    }\n\n    #[inline]\n    fn write_u8(&mut\
    \ self, i: u8) {\n        self.hash.hash_word(i as u32);\n    }\n\n    #[inline]\n\
    \    fn write_u16(&mut self, i: u16) {\n        self.hash.hash_word(i as u32);\n\
    \    }\n\n    #[inline]\n    fn write_u32(&mut self, i: u32) {\n        self.hash.hash_word(i);\n\
    \    }\n\n    #[inline]\n    fn write_u64(&mut self, i: u64) {\n        self.hash.hash_word(i\
    \ as u32);\n        self.hash.hash_word((i >> 32) as u32);\n    }\n\n    #[inline]\n\
    \    #[cfg(target_pointer_width = \"32\")]\n    fn write_usize(&mut self, i: usize)\
    \ {\n        self.write_u32(i as u32);\n    }\n\n    #[inline]\n    #[cfg(target_pointer_width\
    \ = \"64\")]\n    fn write_usize(&mut self, i: usize) {\n        self.write_u64(i\
    \ as u64);\n    }\n\n    #[inline]\n    fn finish(&self) -> u64 {\n        self.hash\
    \ as u64\n    }\n}\n\n/// A convenience function for when you need a quick 64-bit\
    \ hash.\n#[inline]\npub fn hash64<T: Hash + ?Sized>(v: &T) -> u64 {\n    let mut\
    \ state = FxHasher64::default();\n    v.hash(&mut state);\n    state.finish()\n\
    }\n\n/// A convenience function for when you need a quick 32-bit hash.\n#[inline]\n\
    pub fn hash32<T: Hash + ?Sized>(v: &T) -> u32 {\n    let mut state = FxHasher32::default();\n\
    \    v.hash(&mut state);\n    state.finish() as u32\n}\n\n/// A convenience function\
    \ for when you need a quick usize hash.\n#[inline]\npub fn hash<T: Hash + ?Sized>(v:\
    \ &T) -> usize {\n    let mut state = FxHasher::default();\n    v.hash(&mut state);\n\
    \    state.finish() as usize\n}"
  dependsOn: []
  isVerificationFile: false
  path: src/fxhash.rs
  requiredBy: []
  timestamp: '2020-11-17 22:01:40+09:00'
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
