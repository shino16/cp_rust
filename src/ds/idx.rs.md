---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::ops::{Deref, DerefMut, Index, IndexMut};\n\npub struct Idxable<T>(pub\
    \ Vec<T>);\n\nimpl<T> Deref for Idxable<T> {\n\ttype Target = Vec<T>;\n\tfn deref(&self)\
    \ -> &Self::Target {\n\t\t&self.0\n\t}\n}\n\nimpl<T> DerefMut for Idxable<T> {\n\
    \tfn deref_mut(&mut self) -> &mut Self::Target {\n\t\t&mut self.0\n\t}\n}\n\n\
    macro_rules! impl_index {\n\t($($idx:ty),*) => { $(\n\t\timpl<T> Index<$idx> for\
    \ Idxable<T> {\n\t\t\ttype Output = T;\n\t\t\tfn index(&self, index: $idx) ->\
    \ &Self::Output {\n\t\t\t\tself.0.index(index as usize)\n\t\t\t}\n\t\t}\n\t\t\
    impl<T> IndexMut<$idx> for Idxable<T> {\n\t\t\tfn index_mut(&mut self, index:\
    \ $idx) -> &mut Self::Output {\n\t\t\t\tself.0.index_mut(index as usize)\n\t\t\
    \t}\n\t\t}\n\t)* }\n}\n\nimpl_index!(i32, i64, i128, isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/idx.rs
  requiredBy: []
  timestamp: '2020-12-21 16:33:52+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/idx.rs
layout: document
redirect_from:
- /library/src/ds/idx.rs
- /library/src/ds/idx.rs.html
title: src/ds/idx.rs
---
