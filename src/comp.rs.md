---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/slice.rs
    title: src/slice.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/comp.rs\n"
  code: "use crate::slice::*;\nuse std::collections::HashMap;\nuse std::hash::Hash;\n\
    use std::ops::Deref;\n\n#[derive(Debug, Clone)]\npub struct Compress<T: Ord>(Vec<T>);\n\
    \nimpl<T: Ord> Compress<T> {\n    pub fn new(mut data: Vec<T>) -> Self {\n   \
    \     data.sort_unstable();\n        data.dedup();\n        Self(data)\n    }\n\
    \    pub fn len(&self) -> usize { self.0.len() }\n    pub fn compress(&self, v:\
    \ &T) -> usize {\n        let i = lower_bound(&self.0, v);\n        debug_assert!(self.0.get(i)\
    \ == Some(v));\n        i\n    }\n    pub fn restore(&self, i: usize) -> &T {\
    \ &self.0[i] }\n    pub fn cache_all(&self) -> HashMap<T, usize> where T: Clone\
    \ + Hash {\n        self.0.iter().cloned().zip(0..).collect()\n    }\n    pub\
    \ fn into_inner(self) -> Vec<T> { self.0 }\n}\n\nimpl<T: Ord> Deref for Compress<T>\
    \ {\n    type Target = [T];\n    fn deref(&self) -> &Self::Target { &self.0 }\n\
    }\n"
  dependsOn:
  - src/slice.rs
  isVerificationFile: false
  path: src/comp.rs
  requiredBy: []
  timestamp: '2021-05-04 17:50:45+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/comp.rs
layout: document
redirect_from:
- /library/src/comp.rs
- /library/src/comp.rs.html
title: src/comp.rs
---
