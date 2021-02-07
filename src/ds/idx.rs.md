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
    RuntimeError: bundler is not specified: src/ds/idx.rs\n"
  code: "use std::ops::{Deref, DerefMut, Index, IndexMut};\n\npub struct Idxable<T>(pub\
    \ Vec<T>);\n\nimpl<T> Deref for Idxable<T> {\n    type Target = Vec<T>;\n    fn\
    \ deref(&self) -> &Self::Target {\n        &self.0\n    }\n}\n\nimpl<T> DerefMut\
    \ for Idxable<T> {\n    fn deref_mut(&mut self) -> &mut Self::Target {\n     \
    \   &mut self.0\n    }\n}\n\nmacro_rules! impl_index {\n    ($($idx:ty),*) =>\
    \ { $(\n        impl<T> Index<$idx> for Idxable<T> {\n            type Output\
    \ = T;\n            fn index(&self, index: $idx) -> &Self::Output {\n        \
    \        self.0.index(index as usize)\n            }\n        }\n        impl<T>\
    \ IndexMut<$idx> for Idxable<T> {\n            fn index_mut(&mut self, index:\
    \ $idx) -> &mut Self::Output {\n                self.0.index_mut(index as usize)\n\
    \            }\n        }\n    )* }\n}\n\nimpl_index!(i32, i64, i128, isize, u32,\
    \ u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/idx.rs
  requiredBy: []
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/idx.rs
layout: document
redirect_from:
- /library/src/ds/idx.rs
- /library/src/ds/idx.rs.html
title: src/ds/idx.rs
---
