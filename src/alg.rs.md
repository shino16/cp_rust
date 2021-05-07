---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':warning:'
    path: src/ds/dst.rs
    title: src/ds/dst.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/fwk.rs
    title: src/ds/fwk.rs
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
    path: src/ds/sparse_table.rs
    title: src/ds/sparse_table.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/swag.rs
    title: src/ds/swag.rs
  - icon: ':warning:'
    path: src/graph/tree/euler_tour.rs
    title: src/graph/tree/euler_tour.rs
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
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg.rs\n"
  code: "// basic algebraic structures\n\npub mod arith;\n\npub trait Monoid<T: Copy>\
    \ {\n    fn unit(&self) -> T;\n    fn op(&self, x: T, y: T) -> T;\n    fn op_to(&self,\
    \ y: T, x: &mut T) { *x = self.op(*x, y); }\n}\n\npub trait Group<T: Copy>: Monoid<T>\
    \ {\n    fn inv(&self, x: T) -> T;\n    fn op_inv_to(&self, y: T, x: &mut T) {\
    \ *x = self.op(*x, self.inv(y)) }\n}\n\npub struct MonoidImpl<T: Copy, Unit: Fn()\
    \ -> T, Op: Fn(T, T) -> T>(pub Unit, pub Op);\n\npub struct GroupImpl<T: Copy,\
    \ Unit: Fn() -> T, Op: Fn(T, T) -> T, Inv>(pub Unit, pub Op, pub Inv)\nwhere\n\
    \    Inv: Fn(T) -> T;\n\nimpl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T> Monoid<T>\
    \ for MonoidImpl<T, Unit, Op> {\n    fn unit(&self) -> T { (self.0)() }\n    fn\
    \ op(&self, x: T, y: T) -> T { (self.1)(x, y) }\n}\n\nimpl<T: Copy, Unit: Fn()\
    \ -> T, Op: Fn(T, T) -> T, Inv> Monoid<T> for GroupImpl<T, Unit, Op, Inv>\nwhere\n\
    \    Inv: Fn(T) -> T,\n{\n    fn unit(&self) -> T { (self.0)() }\n    fn op(&self,\
    \ x: T, y: T) -> T { (self.1)(x, y) }\n}\n\nimpl<T: Copy, Unit: Fn() -> T, Op:\
    \ Fn(T, T) -> T, Inv> Group<T> for GroupImpl<T, Unit, Op, Inv>\nwhere\n    Inv:\
    \ Fn(T) -> T,\n{\n    fn inv(&self, x: T) -> T { (self.2)(x) }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg.rs
  requiredBy:
  - src/graph/tree/euler_tour.rs
  - src/ds/segtree.rs
  - src/ds/sparse_table.rs
  - src/ds/segtree/lazy.rs
  - src/ds/segtree/beats.rs
  - src/ds/swag.rs
  - src/ds/dst.rs
  - src/ds/fwk.rs
  - src/alg/arith.rs
  timestamp: '2021-02-20 13:28:01+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/swag_test.rs
  - test/src/bin/segtree_test.rs
documentation_of: src/alg.rs
layout: document
redirect_from:
- /library/src/alg.rs
- /library/src/alg.rs.html
title: src/alg.rs
---
