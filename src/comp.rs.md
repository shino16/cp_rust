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
  code: "use crate::slice::*;\n\npub struct Compressed<T>(Box<[T]>);\n\nimpl<T: Ord>\
    \ Compressed<T> {\n    pub fn from(mut data: Vec<T>) -> Self {\n        data.sort_unstable();\n\
    \        data.dedup();\n        Self(data.into_boxed_slice())\n    }\n    pub\
    \ fn ask(&self, v: T) -> usize {\n        debug_assert!(self.0.binary_search(&v).is_ok());\n\
    \        self.0.lower_bound(&v)\n    }\n    pub fn restore(&self, i: usize) ->\
    \ &T {\n        &self.0[i]\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/comp.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/comp.rs
layout: document
redirect_from:
- /library/src/comp.rs
- /library/src/comp.rs.html
title: src/comp.rs
---
