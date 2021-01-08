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
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/cmp/total.rs\n"
  code: "use std::ops::{Deref, DerefMut};\n\n#[repr(transparent)]\n#[derive(PartialEq,\
    \ PartialOrd)]\npub struct Total<T>(pub T);\n\nimpl<T: PartialEq> Eq for Total<T>\
    \ {}\n\nimpl<T: PartialOrd> Ord for Total<T> {\n\tfn cmp(&self, rhs: &Self) ->\
    \ std::cmp::Ordering {\n\t\tself.0.partial_cmp(&rhs.0).unwrap()\n\t}\n}\n\nimpl<T>\
    \ Deref for Total<T> {\n\ttype Target = T;\n\tfn deref(&self) -> &Self::Target\
    \ {\n\t\t&self.0\n\t}\n}\n\nimpl<T> DerefMut for Total<T> {\n\tfn deref_mut(&mut\
    \ self) -> &mut Self::Target {\n\t\t&mut self.0\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/cmp/total.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/cmp/total.rs
layout: document
redirect_from:
- /library/src/cmp/total.rs
- /library/src/cmp/total.rs.html
title: src/cmp/total.rs
---
