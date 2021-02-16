---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':x:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
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
    RuntimeError: bundler is not specified: src/alg/arith.rs\n"
  code: "pub use super::*;\npub use crate::num::*;\n\n#[derive(Default)]\npub struct\
    \ Addition();\n\nimpl<T: Num> Monoid<T> for Addition {\n    fn unit(&self) ->\
    \ T { T::ZERO }\n    fn op(&self, x: T, y: T) -> T { x.wrapping_add(y) }\n}\n\
    impl<T: Num> Group<T> for Addition {\n    fn inv(&self, x: T) -> T { x.wrapping_neg()\
    \ }\n}\n"
  dependsOn:
  - src/alg.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/alg/arith.rs
  requiredBy:
  - src/slice/cum.rs
  - src/ds/fenwick.rs
  - src/graph/euler_tour.rs
  timestamp: '2021-02-16 22:11:19+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/alg/arith.rs
layout: document
redirect_from:
- /library/src/alg/arith.rs
- /library/src/alg/arith.rs.html
title: src/alg/arith.rs
---
