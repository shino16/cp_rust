---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::ops::{Deref, DerefMut};\n\n#[derive(Clone)]\n#[repr(transparent)]\n\
    pub struct UVec<T>(pub Vec<T>);\n\nimpl<T> UVec<T> {\n    pub fn new() -> Self\
    \ {\n        Self(Vec::new())\n    }\n}\n\nimpl<T> AsRef<UVec<T>> for Vec<T> {\n\
    \    fn as_ref(&self) -> &UVec<T> {\n        unsafe { &*(self as *const Vec<T>\
    \ as *const UVec<T>) }\n    }\n}\n\nimpl<T> AsMut<UVec<T>> for Vec<T> {\n    fn\
    \ as_mut(&mut self) -> &mut UVec<T> {\n        unsafe { &mut *(self as *mut Vec<T>\
    \ as *mut UVec<T>) }\n    }\n}\n\nimpl<T> AsRef<Vec<T>> for UVec<T> {\n    fn\
    \ as_ref(&self) -> &Vec<T> {\n        &self.0\n    }\n}\n\nimpl<T> AsMut<Vec<T>>\
    \ for UVec<T> {\n    fn as_mut(&mut self) -> &mut Vec<T> {\n        &mut self.0\n\
    \    }\n}\n\nimpl<T> Deref for UVec<T> {\n    type Target = Vec<T>;\n    fn deref(&self)\
    \ -> &Self::Target {\n        &self.0\n    }\n}\n\nimpl<T> DerefMut for UVec<T>\
    \ {\n    fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut self.0\n\
    \    }\n}\n\n#[cfg(not(debug_assertions))]\nuse std::ops::{Index, IndexMut};\n\
    #[cfg(not(debug_assertions))]\nuse std::slice::SliceIndex;\n\n#[cfg(not(debug_assertions))]\n\
    impl<T, I: SliceIndex<[T]>> Index<I> for UVec<T> {\n    type Output = I::Output;\n\
    \    fn index(&self, index: I) -> &Self::Output {\n        unsafe { self.0.get_unchecked(index)\
    \ }\n    }\n}\n\n#[cfg(not(debug_assertions))]\nimpl<T, I: SliceIndex<[T]>> IndexMut<I>\
    \ for UVec<T> {\n    fn index_mut(&mut self, index: I) -> &mut Self::Output {\n\
    \        unsafe { self.0.get_unchecked_mut(index) }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uvec.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-11-16 03:39:01+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/uvec.rs
layout: document
redirect_from:
- /library/src/ds/uvec.rs
- /library/src/ds/uvec.rs.html
title: src/ds/uvec.rs
---
