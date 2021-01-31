---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
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
  - icon: ':heavy_check_mark:'
    path: test/src/bin/tree_dfs_io_test.rs
    title: test/src/bin/tree_dfs_io_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg/arith.rs\n"
  code: "pub use super::*;\npub use crate::num::*;\nuse std::marker::PhantomData;\n\
    \n#[derive(Default)]\npub struct Addition<N>(PhantomData<N>);\n\nimpl<N> Addition<N>\
    \ {\n\tpub fn new() -> Self {\n\t\tSelf(PhantomData)\n\t}\n}\n\nimpl<N: Num> Alg\
    \ for Addition<N> {\n\ttype Item = N;\n}\n\nimpl<N: Num> Monoid for Addition<N>\
    \ {\n\tfn unit(&self) -> Self::Item {\n\t\tN::ZERO\n\t}\n\tfn op(&self, x: Self::Item,\
    \ y: Self::Item) -> Self::Item {\n\t\tx.wrapping_add(y)\n\t}\n}\n\nimpl<N: Num>\
    \ Group for Addition<N> {\n\tfn inv(&self, x: Self::Item) -> Self::Item {\n\t\t\
    x.wrapping_neg()\n\t}\n}\n"
  dependsOn:
  - src/alg.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/alg/arith.rs
  requiredBy:
  - src/slice/cum.rs
  - src/graph/euler_tour.rs
  - src/ds/fenwick.rs
  timestamp: '2021-01-30 12:54:22+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/tree_dfs_io_test.rs
documentation_of: src/alg/arith.rs
layout: document
redirect_from:
- /library/src/alg/arith.rs
- /library/src/alg/arith.rs.html
title: src/alg/arith.rs
---
