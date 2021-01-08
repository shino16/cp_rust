---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/ds/disjointst.rs
    title: src/ds/disjointst.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/ds/sparsetable.rs
    title: src/ds/sparsetable.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
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
  - src/slice/cum.rs
  - src/ds/disjointst.rs
  - src/ds/sparsetable.rs
  - src/ds/fenwick.rs
  - src/u64/conv.rs
  - src/tests.rs
  - src/int.rs
  - src/math/pow.rs
  - src/math/binom.rs
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/bit.rs
layout: document
redirect_from:
- /library/src/bit.rs
- /library/src/bit.rs.html
title: src/bit.rs
---
