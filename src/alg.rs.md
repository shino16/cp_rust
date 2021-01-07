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
  code: "pub mod action;\npub mod arith;\n\npub trait Alg {\n\ttype Item: Copy;\n\
    }\n\npub trait Monoid: Alg {\n\tfn unit(&self) -> Self::Item;\n\tfn op(&self,\
    \ x: Self::Item, y: Self::Item) -> Self::Item;\n\tfn op_to(&self, y: Self::Item,\
    \ x: &mut Self::Item) {\n\t\t*x = self.op(*x, y);\n\t}\n}\n\npub trait Group:\
    \ Monoid {\n\tfn inv(&self, x: Self::Item) -> Self::Item;\n\tfn op_inv_to(&self,\
    \ y: Self::Item, x: &mut Self::Item) {\n\t\t*x = self.op(*x, self.inv(y))\n\t\
    }\n}\n\nmacro_rules! impl_monoid {\n\t($target:ty, $($params:tt : $bounds:tt),*)\
    \ => {\n\t\timpl<$($params : $bounds),*> Alg for $target {\n\t\t\ttype Item =\
    \ T;\n\t\t}\n\t\timpl<$($params : $bounds),*> Monoid for $target {\n\t\t\tfn unit(&self)\
    \ -> Self::Item {\n\t\t\t\t(self.0)()\n\t\t\t}\n\t\t\tfn op(&self, x: Self::Item,\
    \ y: Self::Item) -> Self::Item {\n\t\t\t\t(self.1)(x, y)\n\t\t\t}\n\t\t}\n\t};\n\
    }\n\nmacro_rules! impl_group {\n\t($target:ty, $($params:tt : $bounds:tt),*) =>\
    \ {\n\t\timpl_monoid!($target, $($params : $bounds),*);\n\t\timpl<$($params :\
    \ $bounds),*> Group for $target {\n\t\t\tfn inv(&self, x: Self::Item) -> Self::Item\
    \ {\n\t\t\t\t(self.2)(x)\n\t\t\t}\n\t\t}\n\t};\n}\n\npub struct MonoidImpl<T:\
    \ Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T>(pub Unit, pub Op);\npub struct GroupImpl<T,\
    \ Unit, Op, Inv>(pub Unit, pub Op, pub Inv)\nwhere\n\tT: Copy,\n\tUnit: Fn() ->\
    \ T,\n\tOp: Fn(T, T) -> T,\n\tInv: Fn(T) -> T;\n\n// help!\nimpl_monoid!(MonoidImpl<T,\
    \ Unit, Op>, T: Copy, Unit: (Fn() -> T), Op: (Fn(T, T) -> T));\nimpl_group!(GroupImpl<T,\
    \ Unit, Op, Inv>,\n\t\t\tT: Copy, Unit: (Fn() -> T), Op: (Fn(T, T) -> T), Inv:\
    \ (Fn(T) -> T));\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
