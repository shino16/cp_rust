---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':warning:'
    path: src/ds/disjointst.rs
    title: src/ds/disjointst.rs
  - icon: ':x:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree.rs
    title: src/ds/segtree.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree/beats.rs
    title: src/ds/segtree/beats.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/segtree/lazy.rs
    title: src/ds/segtree/lazy.rs
  - icon: ':warning:'
    path: src/ds/sparsetable.rs
    title: src/ds/sparsetable.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/swag.rs
    title: src/ds/swag.rs
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
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_test.rs
    title: test/src/bin/segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/swag_test.rs
    title: test/src/bin/swag_test.rs
  - icon: ':x:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg.rs\n"
  code: "// basic algebraic structures\n\npub mod arith;\n\npub trait Monoid {\n \
    \   type Item: Copy;\n    fn unit(&self) -> Self::Item;\n    fn op(&self, x: Self::Item,\
    \ y: Self::Item) -> Self::Item;\n    fn op_to(&self, y: Self::Item, x: &mut Self::Item)\
    \ { *x = self.op(*x, y); }\n}\n\npub trait Group: Monoid {\n    fn inv(&self,\
    \ x: Self::Item) -> Self::Item;\n    fn op_inv_to(&self, y: Self::Item, x: &mut\
    \ Self::Item) { *x = self.op(*x, self.inv(y)) }\n}\n\nmacro_rules! impl_monoid\
    \ {\n    ($target:ty, $($params:tt : $bounds:tt),*) => {\n        impl<$($params\
    \ : $bounds),*> Monoid for $target {\n            type Item = T;\n           \
    \ fn unit(&self) -> Self::Item { (self.0)() }\n            fn op(&self, x: Self::Item,\
    \ y: Self::Item) -> Self::Item { (self.1)(x, y) }\n        }\n    };\n}\n\nmacro_rules!\
    \ impl_group {\n    ($target:ty, $($params:tt : $bounds:tt),*) => {\n        impl_monoid!($target,\
    \ $($params : $bounds),*);\n        impl<$($params : $bounds),*> Group for $target\
    \ {\n            fn inv(&self, x: Self::Item) -> Self::Item { (self.2)(x) }\n\
    \        }\n    };\n}\n\npub struct MonoidImpl<T: Copy, Unit: Fn() -> T, Op: Fn(T,\
    \ T) -> T>(pub Unit, pub Op);\npub struct GroupImpl<T, Unit, Op, Inv>(pub Unit,\
    \ pub Op, pub Inv)\nwhere\n    T: Copy,\n    Unit: Fn() -> T,\n    Op: Fn(T, T)\
    \ -> T,\n    Inv: Fn(T) -> T;\n\nimpl_monoid!(MonoidImpl<T, Unit, Op>, T: Copy,\
    \ Unit: (Fn() -> T), Op: (Fn(T, T) -> T));\nimpl_group!(GroupImpl<T, Unit, Op,\
    \ Inv>,\n            T: Copy, Unit: (Fn() -> T), Op: (Fn(T, T) -> T), Inv: (Fn(T)\
    \ -> T));\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg.rs
  requiredBy:
  - src/alg/arith.rs
  - src/slice/cum.rs
  - src/graph/tree/reroot.rs
  - src/graph/euler_tour.rs
  - src/ds/disjointst.rs
  - src/ds/sparsetable.rs
  - src/ds/swag.rs
  - src/ds/segtree/beats.rs
  - src/ds/segtree/lazy.rs
  - src/ds/segtree.rs
  - src/ds/fenwick.rs
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/segtree_test.rs
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/swag_test.rs
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
