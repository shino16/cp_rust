---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/action.rs
    title: src/alg/action.rs
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':warning:'
    path: src/ds/disjointst.rs
    title: src/ds/disjointst.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree.rs
    title: src/ds/segtree.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree/lazy.rs
    title: src/ds/segtree/lazy.rs
  - icon: ':warning:'
    path: src/ds/sparsetable.rs
    title: src/ds/sparsetable.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  - icon: ':warning:'
    path: src/graph/tree/reroot.rs
    title: src/graph/tree/reroot.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg.rs\n"
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
  requiredBy:
  - src/slice/cum.rs
  - src/ds/segtree/lazy.rs
  - src/ds/disjointst.rs
  - src/ds/sparsetable.rs
  - src/ds/fenwick.rs
  - src/ds/segtree.rs
  - src/graph/tree/reroot.rs
  - src/graph/euler_tour.rs
  - src/alg/action.rs
  - src/alg/arith.rs
  timestamp: '2021-01-03 22:19:36+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_test.rs
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
