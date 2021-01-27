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
    path: src/draft/graph/path.rs
    title: src/draft/graph/path.rs
  - icon: ':warning:'
    path: src/ds/disjointst.rs
    title: src/ds/disjointst.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/ds/sparsetable.rs
    title: src/ds/sparsetable.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':x:'
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
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/bit.rs\n"
  code: "use std::ops::*;\n\npub trait Bits:\n\tSized\n\t+ BitAnd<Output = Self> +\
    \ BitAndAssign\n\t+ BitOr<Output = Self> + BitOrAssign\n\t+ BitXor<Output = Self>\
    \ + BitXorAssign\n\t+ Shl<u32, Output = Self> + ShlAssign<u32>\n\t+ Shr<u32, Output\
    \ = Self> + ShrAssign<u32>\n\t+ Not<Output = Self>\n{\n\tfn trailing_zeros(self)\
    \ -> u32;\n\tfn lsb(self) -> Self;\n\tfn ilog2(self) -> u32;\n\tfn msb(self) ->\
    \ Self;\n}\n\nmacro_rules! impl_bit {\n\t($($t:ty), *) => { $(\n\t\timpl Bits\
    \ for $t {\n\t\t\tfn trailing_zeros(self) -> u32 {\n\t\t\t\t<$t>::trailing_zeros(self)\n\
    \t\t\t}\n\t\t\tfn lsb(self) -> Self {\n\t\t\t\tself & self.wrapping_neg()\n\t\t\
    \t}\n\t\t\tfn ilog2(self) -> u32 {\n\t\t\t\tstd::mem::size_of::<$t>() as u32 *\
    \ 8 - self.leading_zeros() - 1\n\t\t\t}\n\t\t\tfn msb(self) -> Self {\n\t\t\t\t\
    (1 as $t) << self.ilog2()\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_bit!(i32, i64, i128,\
    \ isize, u32, u64, u128, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: src/bit.rs
  requiredBy:
  - src/ds/disjointst.rs
  - src/ds/sparsetable.rs
  - src/ds/fenwick.rs
  - src/tests.rs
  - src/int.rs
  - src/graph/dijkstra.rs
  - src/int/bisect.rs
  - src/int/inv.rs
  - src/int/gcd.rs
  - src/int/arith.rs
  - src/dfa.rs
  - src/math/pow.rs
  - src/math/binom.rs
  - src/draft/fpacc64.rs
  - src/draft/graph/path.rs
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/cargo_test.rs
documentation_of: src/bit.rs
layout: document
redirect_from:
- /library/src/bit.rs
- /library/src/bit.rs.html
title: src/bit.rs
---
