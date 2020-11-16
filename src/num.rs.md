---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
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
  code: "use crate::as_int::*;\nuse crate::bit::*;\nuse std::ops::*;\n\npub trait\
    \ ZeroOne: Copy + Eq {\n    const ZERO: Self;\n    fn is_zero(self) -> bool {\n\
    \        self == Self::ZERO\n    }\n    const ONE: Self;\n}\n\npub trait Num:\n\
    \    ZeroOne\n    + Add<Output = Self>\n    + AddAssign\n    + Sub<Output = Self>\n\
    \    + SubAssign\n    + Mul<Output = Self>\n    + MulAssign\n    + Div<Output\
    \ = Self>\n    + DivAssign\n{\n}\n\npub trait INum: Num + Neg<Output = Self> {}\n\
    \npub trait Int: Num + Ord + Rem<Output = Self> + RemAssign + Bits + CastInt {\n\
    \    type Unsigned: UInt + CastFrom<Self> + CastTo<Self>;\n    fn rem_euclid(self,\
    \ other: Self::Unsigned) -> Self::Unsigned;\n}\n\npub trait IInt: Int + INum {}\n\
    pub trait UInt: Int {}\n\nmacro_rules! impl_int {\n    (@num $t:ty) => {\n   \
    \     impl ZeroOne for $t {\n            const ZERO: Self = 0;\n            const\
    \ ONE: Self = 1;\n        }\n        impl Num for $t {}\n    };\n    (@int $t:ty,\
    \ $u:ty) => {\n        impl Int for $t {\n            type Unsigned = $u;\n  \
    \          fn rem_euclid(self, other: Self::Unsigned) -> Self::Unsigned {\n  \
    \              <$t>::rem_euclid(self, other as $t) as $u\n            }\n    \
    \    }\n    };\n    ({ $i:ty }, { $u:ty }) => {\n        impl_int!(@num $i);\n\
    \        impl_int!(@num $u);\n        impl_int!(@int $i, $u);\n        impl_int!(@int\
    \ $u, $u);\n        impl INum for $i {}\n        impl IInt for $i {}\n       \
    \ impl UInt for $u {}\n    };\n    ({ $i:ty, $($is:ty),* }, { $u:ty, $($us:ty),*\
    \ }) => {\n        impl_int!({ $i }, { $u });\n        impl_int!({ $($is),* },\
    \ { $($us),* });\n    }\n}\n\nimpl_int!({ i32, i64, i128, isize }, { u32, u64,\
    \ u128, usize });\n"
  dependsOn: []
  isVerificationFile: false
  path: src/num.rs
  requiredBy: []
  timestamp: '2020-11-04 21:05:29+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_test.rs
documentation_of: src/num.rs
layout: document
redirect_from:
- /library/src/num.rs
- /library/src/num.rs.html
title: src/num.rs
---
