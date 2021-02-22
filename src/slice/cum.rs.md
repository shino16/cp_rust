---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
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
    RuntimeError: bundler is not specified: src/slice/cum.rs\n"
  code: "pub use crate::alg::arith::*;\n\npub trait Cum {\n    type Item: Copy;\n\
    \    fn cuml<M: Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item>;\n    fn cumr<M:\
    \ Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item>;\n    fn cuml_sum(&self)\
    \ -> Vec<Self::Item>\n    where\n        Self::Item: Num,\n    {\n        self.cuml(Addition())\n\
    \    }\n    fn cumr_sum(&self) -> Vec<Self::Item>\n    where\n        Self::Item:\
    \ Num,\n    {\n        self.cumr(Addition())\n    }\n}\n\nimpl<T: Copy> Cum for\
    \ [T] {\n    type Item = T;\n    fn cuml<M: Monoid<Self::Item>>(&self, m: M) ->\
    \ Vec<Self::Item> {\n        let mut res = Vec::with_capacity(self.len() + 1);\n\
    \        let mut tl = m.unit();\n        res.push(tl);\n        for e in self\
    \ {\n            tl = m.op(tl, *e);\n            res.push(tl);\n        }\n  \
    \      res\n    }\n\n    fn cumr<M: Monoid<Self::Item>>(&self, m: M) -> Vec<Self::Item>\
    \ {\n        let mut res = Vec::with_capacity(self.len() + 1);\n        let mut\
    \ tl = m.unit();\n        res.push(tl);\n        for e in self.iter().rev() {\n\
    \            tl = m.op(*e, tl);\n            res.push(tl);\n        }\n      \
    \  res.reverse();\n        res\n    }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/slice/cum.rs
  requiredBy: []
  timestamp: '2021-02-20 14:04:23+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice/cum.rs
layout: document
redirect_from:
- /library/src/slice/cum.rs
- /library/src/slice/cum.rs.html
title: src/slice/cum.rs
---
