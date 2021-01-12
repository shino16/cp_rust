---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/graph/path.rs
    title: src/draft/graph/path.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':question:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
  _extendedVerifiedWith:
  - icon: ':x:'
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
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/int.rs\n"
  code: "use crate::bit::*;\nuse crate::cast::*;\npub use crate::zo::*;\nuse std::fmt::*;\n\
    use std::ops::*;\n\npub mod arith;\npub mod bisect;\npub mod gcd;\npub mod inv;\n\
    \npub trait Num:\n\tZeroOne\n\t+ Add<Output = Self> + AddAssign\n\t+ Sub<Output\
    \ = Self> + SubAssign\n\t+ Mul<Output = Self> + MulAssign\n\t+ Div<Output = Self>\
    \ + DivAssign\n\t+ Debug + Display\n{\n\tfn wrapping_add(self, rhs: Self) -> Self\
    \ {\n\t\tself + rhs\n\t}\n\tfn wrapping_neg(self) -> Self;\n}\n\npub trait INum:\
    \ Num + Neg<Output = Self> {}\n\npub trait Int: Num + Ord + Rem<Output = Self>\
    \ + RemAssign + Bits + PrimCast {\n\ttype Signed: IInt + CastFrom<Self> + CastTo<Self>;\n\
    \ttype Unsigned: UInt + CastFrom<Self> + CastTo<Self>;\n\tconst MIN: Self;\n\t\
    const MAX: Self;\n\tfn abs(self) -> Self::Unsigned;\n\tfn rem_euclid(self, rhs:\
    \ Self::Unsigned) -> Self::Unsigned;\n}\n\npub trait IInt: Int + INum {}\npub\
    \ trait UInt: Int {}\n\nmacro_rules! impl_int {\n\t(@num $t:ident) => {\n\t\t\
    impl Num for $t {\n\t\t\tfn wrapping_add(self, rhs: Self) -> Self {\n\t\t\t\t\
    self.wrapping_add(rhs)\n\t\t\t}\n\t\t\tfn wrapping_neg(self) -> Self {\n\t\t\t\
    \tself.wrapping_neg()\n\t\t\t}\n\t\t}\n\t};\n\t(@int $t:ident, $i:ident, $u:ident)\
    \ => {\n\t\timpl Int for $t {\n\t\t\ttype Signed = $i;\n\t\t\ttype Unsigned =\
    \ $u;\n\t\t\tconst MIN: Self = std::$t::MIN;\n\t\t\tconst MAX: Self = std::$t::MAX;\n\
    \t\t\t#[allow(unconditional_recursion)] // it's not\n\t\t\tfn abs(self) -> Self::Unsigned\
    \ {\n\t\t\t\tself.abs() as $u\n\t\t\t}\n\t\t\tfn rem_euclid(self, rhs: Self::Unsigned)\
    \ -> Self::Unsigned {\n\t\t\t\t<$t>::rem_euclid(self, rhs as $t) as $u\n\t\t\t\
    }\n\t\t}\n\t};\n\t({ $i:ident }, { $u:ident }) => {\n\t\timpl_int!(@num $i);\n\
    \t\timpl_int!(@num $u);\n\t\timpl_int!(@int $i, $i, $u);\n\t\timpl_int!(@int $u,\
    \ $i, $u);\n\t\timpl INum for $i {}\n\t\timpl IInt for $i {}\n\t\timpl UInt for\
    \ $u {}\n\t};\n\t({ $i:ident, $($is:ident),* }, { $u:ident, $($us:ident),* })\
    \ => {\n\t\timpl_int!({ $i }, { $u });\n\t\timpl_int!({ $($is),* }, { $($us),*\
    \ });\n\t}\n}\n\nimpl_int!({ i32, i64, i128, isize }, { u32, u64, u128, usize\
    \ });\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/int.rs
  requiredBy:
  - src/fp.rs
  - src/slice/cum.rs
  - src/ds/fenwick.rs
  - src/u64/conv.rs
  - src/tests.rs
  - src/dfa.rs
  - src/mint.rs
  - src/math/pow.rs
  - src/math/binom.rs
  - src/draft/graph/path.rs
  timestamp: '2020-12-21 20:11:53+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/int.rs
layout: document
redirect_from:
- /library/src/int.rs
- /library/src/int.rs.html
title: src/int.rs
---
