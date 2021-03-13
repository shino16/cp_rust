---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/alg.rs
    title: src/alg.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg/arith.rs\n"
  code: "pub use super::*;\npub use crate::num::*;\n\n#[derive(Default, Clone, Copy)]\n\
    pub struct Addition();\n\nmacro_rules! impl_alg {\n    ($($t:ty),*) => { $(\n\
    \        impl Monoid<$t> for Addition {\n            fn unit(&self) -> $t { 0\
    \ }\n            fn op(&self, x: $t, y: $t) -> $t { x.wrapping_add(y) }\n    \
    \    }\n        impl Group<$t> for Addition {\n            fn inv(&self, x: $t)\
    \ -> $t { x.wrapping_neg() }\n        }\n    )* };\n}\n\nimpl_alg!(i8, i16, i32,\
    \ i64, isize, u8, u16, u32, u64, usize);\n"
  dependsOn:
  - src/alg.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/alg/arith.rs
  requiredBy:
  - src/graph/euler_tour.rs
  - src/ds/fenwick.rs
  timestamp: '2021-02-24 00:44:23+09:00'
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
