---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Deref, DerefMut};\n\n#[derive(Clone)]\n#[repr(transparent)]\n\
    pub struct UVec<T>(pub Vec<T>);\n\nimpl<T> UVec<T> {\n\tpub fn new() -> Self {\n\
    \t\tSelf(Vec::new())\n\t}\n}\n\nimpl<T> AsRef<UVec<T>> for Vec<T> {\n\tfn as_ref(&self)\
    \ -> &UVec<T> {\n\t\tunsafe { &*(self as *const Vec<T> as *const UVec<T>) }\n\t\
    }\n}\n\nimpl<T> AsMut<UVec<T>> for Vec<T> {\n\tfn as_mut(&mut self) -> &mut UVec<T>\
    \ {\n\t\tunsafe { &mut *(self as *mut Vec<T> as *mut UVec<T>) }\n\t}\n}\n\nimpl<T>\
    \ AsRef<Vec<T>> for UVec<T> {\n\tfn as_ref(&self) -> &Vec<T> {\n\t\t&self.0\n\t\
    }\n}\n\nimpl<T> AsMut<Vec<T>> for UVec<T> {\n\tfn as_mut(&mut self) -> &mut Vec<T>\
    \ {\n\t\t&mut self.0\n\t}\n}\n\nimpl<T> Deref for UVec<T> {\n\ttype Target = Vec<T>;\n\
    \tfn deref(&self) -> &Self::Target {\n\t\t&self.0\n\t}\n}\n\nimpl<T> DerefMut\
    \ for UVec<T> {\n\tfn deref_mut(&mut self) -> &mut Self::Target {\n\t\t&mut self.0\n\
    \t}\n}\n\n#[cfg(not(debug_assertions))]\nuse std::ops::{Index, IndexMut};\n#[cfg(not(debug_assertions))]\n\
    use std::slice::SliceIndex;\n\n#[cfg(not(debug_assertions))]\nimpl<T, I: SliceIndex<[T]>>\
    \ Index<I> for UVec<T> {\n\ttype Output = I::Output;\n\tfn index(&self, index:\
    \ I) -> &Self::Output {\n\t\tunsafe { self.0.get_unchecked(index) }\n\t}\n}\n\n\
    #[cfg(not(debug_assertions))]\nimpl<T, I: SliceIndex<[T]>> IndexMut<I> for UVec<T>\
    \ {\n\tfn index_mut(&mut self, index: I) -> &mut Self::Output {\n\t\tunsafe {\
    \ self.0.get_unchecked_mut(index) }\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uvec.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/uvec.rs
layout: document
redirect_from:
- /library/src/ds/uvec.rs
- /library/src/ds/uvec.rs.html
title: src/ds/uvec.rs
---
