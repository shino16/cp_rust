---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait VecDim<Elem> {\n\ttype Item;\n\tfn make_vec(self, elem: Elem) ->\
    \ Vec<Self::Item>;\n}\n\nimpl<Elem: Clone> VecDim<Elem> for usize {\n\ttype Item\
    \ = Elem;\n\tfn make_vec(self, elem: Elem) -> Vec<Self::Item> {\n\t\tvec![elem;\
    \ self]\n\t}\n}\n\nimpl<Elem, Dim> VecDim<Elem> for (usize, Dim)\nwhere\n\tDim:\
    \ VecDim<Elem>,\n\tDim::Item: Clone,\n{\n\ttype Item = Vec<Dim::Item>;\n\tfn make_vec(self,\
    \ elem: Elem) -> Vec<Self::Item> {\n\t\tvec![self.1.make_vec(elem); self.0]\n\t\
    }\n}\n\npub fn make_vec<Elem, Dim: VecDim<Elem>>(dim: Dim, elem: Elem) -> Vec<Dim::Item>\
    \ {\n\tdim.make_vec(elem)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/make_vec.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/make_vec.rs
layout: document
redirect_from:
- /library/src/make_vec.rs
- /library/src/make_vec.rs.html
title: src/make_vec.rs
---
