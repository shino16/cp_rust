---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
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
  code: "pub use crate::alg::arith::*;\n\npub trait Cum {\n\ttype Item: Copy;\n\t\
    fn cuml<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>;\n\tfn cumr<M:\
    \ Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>;\n\tfn cuml_sum(&self)\
    \ -> Vec<Self::Item>\n\twhere\n\t\tSelf::Item: Num,\n\t{\n\t\tself.cuml(Addition::new())\n\
    \t}\n\tfn cumr_sum(&self) -> Vec<Self::Item>\n\twhere\n\t\tSelf::Item: Num,\n\t\
    {\n\t\tself.cumr(Addition::new())\n\t}\n}\n\nimpl<T: Copy> Cum for [T] {\n\ttype\
    \ Item = T;\n\tfn cuml<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>\
    \ {\n\t\tlet mut res = Vec::with_capacity(self.len() + 1);\n\t\tlet mut tl = m.unit();\n\
    \t\tres.push(tl.clone());\n\t\tfor e in self {\n\t\t\ttl = m.op(tl, e.clone());\n\
    \t\t\tres.push(tl.clone());\n\t\t}\n\t\tres\n\t}\n\n\tfn cumr<M: Monoid<Item =\
    \ Self::Item>>(&self, m: M) -> Vec<Self::Item> {\n\t\tlet mut res = Vec::with_capacity(self.len()\
    \ + 1);\n\t\tlet mut tl = m.unit();\n\t\tres.push(tl.clone());\n\t\tfor e in self.into_iter().rev()\
    \ {\n\t\t\ttl = m.op(e.clone(), tl);\n\t\t\tres.push(tl.clone());\n\t\t}\n\t\t\
    res.reverse();\n\t\tres\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  - src/alg/arith.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/slice/cum.rs
  requiredBy: []
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice/cum.rs
layout: document
redirect_from:
- /library/src/slice/cum.rs
- /library/src/slice/cum.rs.html
title: src/slice/cum.rs
---
