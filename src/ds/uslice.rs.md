---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::ops::{Deref, DerefMut};\n\n#[repr(transparent)]\npub struct USlice<T>(pub\
    \ [T]);\n\nimpl<T> AsRef<USlice<T>> for [T] {\n\tfn as_ref(&self) -> &USlice<T>\
    \ {\n\t\tunsafe { &*(self as *const [T] as *const USlice<T>) }\n\t}\n}\n\nimpl<T>\
    \ AsMut<USlice<T>> for [T] {\n\tfn as_mut(&mut self) -> &mut USlice<T> {\n\t\t\
    unsafe { &mut *(self as *mut [T] as *mut USlice<T>) }\n\t}\n}\n\nimpl<T> AsRef<[T]>\
    \ for USlice<T> {\n\tfn as_ref(&self) -> &[T] {\n\t\t&self.0\n\t}\n}\n\nimpl<T>\
    \ AsMut<[T]> for USlice<T> {\n\tfn as_mut(&mut self) -> &mut [T] {\n\t\t&mut self.0\n\
    \t}\n}\n\nimpl<T> Deref for USlice<T> {\n\ttype Target = [T];\n\tfn deref(&self)\
    \ -> &Self::Target {\n\t\t&self.0\n\t}\n}\n\nimpl<T> DerefMut for USlice<T> {\n\
    \tfn deref_mut(&mut self) -> &mut Self::Target {\n\t\t&mut self.0\n\t}\n}\n\n\
    #[cfg(not(debug_assertions))]\nuse std::ops::{Index, IndexMut};\n\n#[cfg(not(debug_assertions))]\n\
    impl<T> Index<usize> for USlice<T> {\n\ttype Output = T;\n\tfn index(&self, index:\
    \ usize) -> &Self::Output {\n\t\tunsafe { self.0.get_unchecked(index) }\n\t}\n\
    }\n\n#[cfg(not(debug_assertions))]\nimpl<T> IndexMut<usize> for USlice<T> {\n\t\
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {\n\t\tunsafe { self.0.get_unchecked_mut(index)\
    \ }\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uslice.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/uslice.rs
layout: document
redirect_from:
- /library/src/ds/uslice.rs
- /library/src/ds/uslice.rs.html
title: src/ds/uslice.rs
---
