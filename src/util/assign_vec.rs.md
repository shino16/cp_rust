---
data:
  _extendedDependsOn: []
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
    RuntimeError: bundler is not specified: src/util/assign_vec.rs\n"
  code: "pub trait Assign {\n    type Item: Clone;\n    fn assign(&mut self, e: Self::Item,\
    \ len: usize);\n}\n\nimpl<T: Clone> Assign for Vec<T> {\n    type Item = T;\n\
    \    fn assign(&mut self, e: T, len: usize) {\n        self.clear();\n       \
    \ self.extend((0..len).map(|_| e.clone()));\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/util/assign_vec.rs
  requiredBy: []
  timestamp: '2021-03-19 19:57:04+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/util/assign_vec.rs
layout: document
redirect_from:
- /library/src/util/assign_vec.rs
- /library/src/util/assign_vec.rs.html
title: src/util/assign_vec.rs
---
