---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':warning:'
    path: src/ds/disjointst.rs
    title: src/ds/disjointst.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/ds/sparsetable.rs
    title: src/ds/sparsetable.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':heavy_check_mark:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':warning:'
    path: src/int/inv.rs
    title: src/int/inv.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/segtree_beats_test.rs
    title: test/src/bin/segtree_beats_test.rs
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
    RuntimeError: bundler is not specified: src/bits.rs\n"
  code: "use std::ops::*;\n\npub trait Bits:\n    Sized\n    + BitAnd<Output = Self>\
    \ + BitAndAssign\n    + BitOr<Output = Self> + BitOrAssign\n    + BitXor<Output\
    \ = Self> + BitXorAssign\n    + Shl<u32, Output = Self> + ShlAssign<u32>\n   \
    \ + Shr<u32, Output = Self> + ShrAssign<u32>\n    + Not<Output = Self>\n{\n  \
    \  fn trailing_zeros(self) -> u32;\n    fn lsb(self) -> Self;\n    fn ilog2(self)\
    \ -> u32;\n    fn msb(self) -> Self;\n}\n\nmacro_rules! impl_bit {\n    ($($t:ty),\
    \ *) => { $(\n        impl Bits for $t {\n            fn trailing_zeros(self)\
    \ -> u32 {\n                <$t>::trailing_zeros(self)\n            }\n      \
    \      fn lsb(self) -> Self {\n                self & self.wrapping_neg()\n  \
    \          }\n            fn ilog2(self) -> u32 {\n                std::mem::size_of::<$t>()\
    \ as u32 * 8 - self.leading_zeros() - 1\n            }\n            fn msb(self)\
    \ -> Self {\n                (1 as $t) << self.ilog2()\n            }\n      \
    \  }\n    )* };\n}\n\nimpl_bit!(i32, i64, i128, isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bits.rs
  requiredBy:
  - src/dfa.rs
  - src/math/binom.rs
  - src/math/pow.rs
  - src/draft/fpacc64.rs
  - src/ds/fenwick.rs
  - src/ds/disjointst.rs
  - src/ds/sparsetable.rs
  - src/int/inv.rs
  - src/int/bisect.rs
  - src/int/gcd.rs
  - src/int/arith.rs
  - src/tests.rs
  - src/graph/dijkstra.rs
  - src/int.rs
  timestamp: '2021-02-10 04:47:06+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/segtree_beats_test.rs
  - test/src/bin/tree_dfs_io_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/bits.rs
layout: document
redirect_from:
- /library/src/bits.rs
- /library/src/bits.rs.html
title: src/bits.rs
---
