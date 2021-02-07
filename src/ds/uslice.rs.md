---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/ds/uslice.rs\n"
  code: "use std::ops::{Deref, DerefMut};\n\n#[repr(transparent)]\npub struct USlice<T>(pub\
    \ [T]);\n\nimpl<T> AsRef<USlice<T>> for [T] {\n    fn as_ref(&self) -> &USlice<T>\
    \ {\n        unsafe { &*(self as *const [T] as *const USlice<T>) }\n    }\n}\n\
    \nimpl<T> AsMut<USlice<T>> for [T] {\n    fn as_mut(&mut self) -> &mut USlice<T>\
    \ {\n        unsafe { &mut *(self as *mut [T] as *mut USlice<T>) }\n    }\n}\n\
    \nimpl<T> AsRef<[T]> for USlice<T> {\n    fn as_ref(&self) -> &[T] {\n       \
    \ &self.0\n    }\n}\n\nimpl<T> AsMut<[T]> for USlice<T> {\n    fn as_mut(&mut\
    \ self) -> &mut [T] {\n        &mut self.0\n    }\n}\n\nimpl<T> Deref for USlice<T>\
    \ {\n    type Target = [T];\n    fn deref(&self) -> &Self::Target {\n        &self.0\n\
    \    }\n}\n\nimpl<T> DerefMut for USlice<T> {\n    fn deref_mut(&mut self) ->\
    \ &mut Self::Target {\n        &mut self.0\n    }\n}\n\n#[cfg(not(debug_assertions))]\n\
    use std::ops::{Index, IndexMut};\n\n#[cfg(not(debug_assertions))]\nimpl<T> Index<usize>\
    \ for USlice<T> {\n    type Output = T;\n    fn index(&self, index: usize) ->\
    \ &Self::Output {\n        unsafe { self.0.get_unchecked(index) }\n    }\n}\n\n\
    #[cfg(not(debug_assertions))]\nimpl<T> IndexMut<usize> for USlice<T> {\n    fn\
    \ index_mut(&mut self, index: usize) -> &mut Self::Output {\n        unsafe {\
    \ self.0.get_unchecked_mut(index) }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uslice.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/uslice.rs
layout: document
redirect_from:
- /library/src/ds/uslice.rs
- /library/src/ds/uslice.rs.html
title: src/ds/uslice.rs
---
