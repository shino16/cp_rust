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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/hash.rs\n"
  code: "use crate::slice::*;\nuse std::collections::HashMap;\nuse std::hash::Hash;\n\
    \npub struct Compress<T: Ord>(Vec<T>);\n\nimpl<T: Ord> Compress<T> {\n    pub\
    \ fn from(mut data: Vec<T>) -> Self {\n        data.sort_unstable();\n       \
    \ data.dedup();\n        Self(data)\n    }\n    pub fn compress(&self, v: &T)\
    \ -> usize {\n        debug_assert!(self.0.binary_search(v).is_ok());\n      \
    \  lower_bound(&self.0, v)\n    }\n    pub fn restore(&self, i: usize) -> &T {\n\
    \        &self.0[i]\n    }\n    pub fn cache_al(&self) -> HashMap<T, usize>\n\
    \    where\n        T: Clone + Hash,\n    {\n        self.0.iter().cloned().zip(0..).collect()\n\
    \    }\n}\n"
  dependsOn:
  - src/slice.rs
  isVerificationFile: false
  path: src/hash.rs
  requiredBy: []
  timestamp: '2021-03-14 02:25:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/hash.rs
layout: document
redirect_from:
- /library/src/hash.rs
- /library/src/hash.rs.html
title: src/hash.rs
---
