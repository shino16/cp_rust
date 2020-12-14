---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::as_int::*;\nuse crate::bit::*;\nuse std::fmt::*;\nuse std::ops::*;\n\
    \npub mod bisect;\n\npub trait ZeroOne: Copy + Eq {\n\tconst ZERO: Self;\n\tfn\
    \ is_zero(self) -> bool {\n\t\tself == Self::ZERO\n\t}\n\tconst ONE: Self;\n}\n\
    \npub trait Num:\n\tZeroOne\n\t+ Add<Output = Self> + AddAssign\n\t+ Sub<Output\
    \ = Self> + SubAssign\n\t+ Mul<Output = Self> + MulAssign\n\t+ Div<Output = Self>\
    \ + DivAssign\n\t+ Debug + Display\n{\n\tfn wrapping_neg(self) -> Self;\n}\n\n\
    pub trait INum: Num + Neg<Output = Self> {}\n\npub trait Int: Num + Ord + Rem<Output\
    \ = Self> + RemAssign + Bits + CastInt {\n\ttype Signed: IInt + CastFrom<Self>\
    \ + CastTo<Self>;\n\ttype Unsigned: UInt + CastFrom<Self> + CastTo<Self>;\n\t\
    const MIN: Self;\n\tconst MAX: Self;\n\tfn rem_euclid(self, rhs: Self::Unsigned)\
    \ -> Self::Unsigned;\n}\n\npub trait IInt: Int + INum {}\npub trait UInt: Int\
    \ {}\n\nmacro_rules! impl_int {\n\t(@num $t:ident) => {\n\t\timpl ZeroOne for\
    \ $t {\n\t\t\tconst ZERO: Self = 0;\n\t\t\tconst ONE: Self = 1;\n\t\t}\n\t\timpl\
    \ Num for $t {\n\t\t\tfn wrapping_neg(self) -> Self {\n\t\t\t\tself.wrapping_neg()\n\
    \t\t\t}\n\t\t}\n\t};\n\t(@int $t:ident, $i:ident, $u:ident) => {\n\t\timpl Int\
    \ for $t {\n\t\t\ttype Signed = $i;\n\t\t\ttype Unsigned = $u;\n\t\t\tconst MIN:\
    \ Self = std::$t::MIN;\n\t\t\tconst MAX: Self = std::$t::MAX;\n\t\t\tfn rem_euclid(self,\
    \ rhs: Self::Unsigned) -> Self::Unsigned {\n\t\t\t\t<$t>::rem_euclid(self, rhs\
    \ as $t) as $u\n\t\t\t}\n\t\t}\n\t};\n\t({ $i:ident }, { $u:ident }) => {\n\t\t\
    impl_int!(@num $i);\n\t\timpl_int!(@num $u);\n\t\timpl_int!(@int $i, $i, $u);\n\
    \t\timpl_int!(@int $u, $i, $u);\n\t\timpl INum for $i {}\n\t\timpl IInt for $i\
    \ {}\n\t\timpl UInt for $u {}\n\t};\n\t({ $i:ident, $($is:ident),* }, { $u:ident,\
    \ $($us:ident),* }) => {\n\t\timpl_int!({ $i }, { $u });\n\t\timpl_int!({ $($is),*\
    \ }, { $($us),* });\n\t}\n}\n\nimpl_int!({ i32, i64, i128, isize }, { u32, u64,\
    \ u128, usize });\n"
  dependsOn: []
  isVerificationFile: false
  path: src/int.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/int.rs
layout: document
redirect_from:
- /library/src/int.rs
- /library/src/int.rs.html
title: src/int.rs
---
