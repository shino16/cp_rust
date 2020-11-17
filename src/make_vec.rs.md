---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub trait VecDim<Elem> {\n    type Item;\n    fn make_vec(self, elem: Elem)\
    \ -> Vec<Self::Item>;\n}\n\nimpl<Elem: Clone> VecDim<Elem> for usize {\n    type\
    \ Item = Elem;\n    fn make_vec(self, elem: Elem) -> Vec<Self::Item> {\n     \
    \   vec![elem; self]\n    }\n}\n\nimpl<Elem, Dim> VecDim<Elem> for (usize, Dim)\n\
    where\n    Dim: VecDim<Elem>,\n    Dim::Item: Clone,\n{\n    type Item = Vec<Dim::Item>;\n\
    \    fn make_vec(self, elem: Elem) -> Vec<Self::Item> {\n        vec![self.1.make_vec(elem);\
    \ self.0]\n    }\n}\n\npub fn make_vec<Elem, Dim: VecDim<Elem>>(dim: Dim, elem:\
    \ Elem) -> Vec<Dim::Item> {\n    dim.make_vec(elem)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/make_vec.rs
  requiredBy: []
  timestamp: '2020-11-17 16:16:39+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/make_vec.rs
layout: document
redirect_from:
- /library/src/make_vec.rs
- /library/src/make_vec.rs.html
title: src/make_vec.rs
---