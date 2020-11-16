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
  code: "use std::ops::{Deref, DerefMut};\n\n#[derive(PartialEq, PartialOrd)]\npub\
    \ struct Total<T>(pub T);\n\nimpl<T: PartialEq> Eq for Total<T> {}\n\nimpl<T:\
    \ PartialOrd> Ord for Total<T> {\n    fn cmp(&self, other: &Self) -> std::cmp::Ordering\
    \ {\n        self.0.partial_cmp(&other.0).unwrap()\n    }\n}\n\nimpl<T> Deref\
    \ for Total<T> {\n    type Target = T;\n    fn deref(&self) -> &Self::Target {\n\
    \        &self.0\n    }\n}\n\nimpl<T> DerefMut for Total<T> {\n    fn deref_mut(&mut\
    \ self) -> &mut Self::Target {\n        &mut self.0\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/cmp/total.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/cmp/total.rs
layout: document
redirect_from:
- /library/src/cmp/total.rs
- /library/src/cmp/total.rs.html
title: src/cmp/total.rs
---
