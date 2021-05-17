---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds.rs
    title: src/ds.rs
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/ds/sparse_table.rs
    title: src/ds/sparse_table.rs
  - icon: ':warning:'
    path: src/ds/uvec2d.rs
    title: src/ds/uvec2d.rs
  - icon: ':warning:'
    path: src/float/conv.rs
    title: src/float/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/gf/conv.rs
    title: src/gf/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/uvec.rs\n"
  code: "pub use crate::uvec;\nuse std::ops::{Deref, DerefMut};\n\n#[macro_export]\n\
    macro_rules! uvec {\n    ($($t:tt)*) => { UVec(vec![$($t)*]) };\n}\n\n#[derive(Clone)]\n\
    #[repr(transparent)]\npub struct UVec<T>(pub Vec<T>);\n\nimpl<T> UVec<T> { pub\
    \ const fn new() -> Self { Self(Vec::new()) } }\nimpl<T> AsRef<UVec<T>> for Vec<T>\
    \ {\n    fn as_ref(&self) -> &UVec<T> {\n        unsafe { &*(self as *const Vec<T>\
    \ as *const UVec<T>) }\n    }\n}\nimpl<T> AsMut<UVec<T>> for Vec<T> {\n    fn\
    \ as_mut(&mut self) -> &mut UVec<T> {\n        unsafe { &mut *(self as *mut Vec<T>\
    \ as *mut UVec<T>) }\n    }\n}\nimpl<T> AsRef<Vec<T>> for UVec<T> { fn as_ref(&self)\
    \ -> &Vec<T> { &self.0 } }\nimpl<T> AsMut<Vec<T>> for UVec<T> { fn as_mut(&mut\
    \ self) -> &mut Vec<T> { &mut self.0 } }\nimpl<T> Deref for UVec<T> {\n    type\
    \ Target = Vec<T>;\n    fn deref(&self) -> &Self::Target { &self.0 }\n}\nimpl<T>\
    \ DerefMut for UVec<T> {\n    fn deref_mut(&mut self) -> &mut Self::Target { &mut\
    \ self.0 }\n}\n\n#[cfg(not(debug_assertions))]\nmod unchecked {\n    use super::*;\n\
    \    use std::ops::{Index, IndexMut};\n    use std::slice::SliceIndex;\n\n   \
    \ impl<T, I: SliceIndex<[T]>> Index<I> for UVec<T> {\n        type Output = I::Output;\n\
    \        fn index(&self, index: I) -> &Self::Output {\n            unsafe { self.0.get_unchecked(index)\
    \ }\n        }\n    }\n    impl<T, I: SliceIndex<[T]>> IndexMut<I> for UVec<T>\
    \ {\n        fn index_mut(&mut self, index: I) -> &mut Self::Output {\n      \
    \      unsafe { self.0.get_unchecked_mut(index) }\n        }\n    }\n}\n"
  dependsOn:
  - src/lib.rs
  - src/ds.rs
  isVerificationFile: false
  path: src/ds/uvec.rs
  requiredBy:
  - src/u64/conv.rs
  - src/float/conv.rs
  - src/ds/sparse_table.rs
  - src/ds/uvec2d.rs
  - src/mint/conv.rs
  - src/gf/conv.rs
  timestamp: '2021-05-17 11:32:22+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_test.rs
documentation_of: src/ds/uvec.rs
layout: document
redirect_from:
- /library/src/ds/uvec.rs
- /library/src/ds/uvec.rs.html
title: src/ds/uvec.rs
---
