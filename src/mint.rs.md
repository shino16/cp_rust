---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':x:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/mint/num.rs
    title: src/mint/num.rs
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
  - icon: ':x:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':x:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':x:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/mint.rs\n"
  code: "pub use crate::zo::ZeroOne;\nuse std::marker::PhantomData;\nuse std::{fmt,\
    \ iter, ops};\n\npub mod conv;\npub mod num;\n\npub trait Mod: Default + Clone\
    \ + Copy + PartialEq + Eq {\n    const M: u32;\n    const PHI: u32;\n}\n\n#[macro_export]\n\
    macro_rules! def_mod {\n    ($name:ident, $modulus:expr) => {\n        def_mod!($name,\
    \ $modulus, $modulus - 1);\n    };\n    ($name:ident, $modulus:expr, $phi:expr)\
    \ => {\n        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n      \
    \  pub struct $name;\n        impl Mod for $name {\n            const M: u32 =\
    \ $modulus;\n            const PHI: u32 = $phi;\n        }\n    };\n}\n\ndef_mod!(ModA,\
    \ 1_000_000_007);\ndef_mod!(ModB, 998_244_353);\ndef_mod!(ModC, 1_012_924_417);\n\
    def_mod!(ModD, 924_844_033);\n\n#[derive(Default, Clone, Copy, PartialEq, Eq)]\n\
    pub struct Mint<M: Mod> {\n    pub val: u32,\n    _m: PhantomData<M>,\n}\n\npub\
    \ type MintA = Mint<ModA>;\npub type MintB = Mint<ModB>;\npub type MintC = Mint<ModC>;\n\
    pub type MintD = Mint<ModD>;\n\npub type Mint17 = MintA;\npub type Mint99 = MintB;\n\
    \nimpl<M: Mod> Mint<M> {\n    pub const M: u32 = M::M;\n    pub fn new(val: i64)\
    \ -> Self {\n        Self::from_val(val.rem_euclid(M::M as i64) as u32)\n    }\n\
    \    pub fn from_val(val: u32) -> Self {\n        Mint { val, _m: PhantomData\
    \ }\n    }\n    pub fn value(self) -> u32 {\n        self.val\n    }\n    pub\
    \ fn pow(self, mut exp: u32) -> Self {\n        if self.val == 0 && exp == 0 {\n\
    \            return Self::from_val(1);\n        }\n        let mut e = self;\n\
    \        let mut res = Self::from_val(1);\n        while exp != 0 {\n        \
    \    if exp % 2 == 1 {\n                res *= e;\n            }\n           \
    \ e *= e;\n            exp >>= 1;\n        }\n        res\n    }\n    pub fn inv(self)\
    \ -> Self {\n        self.pow(M::PHI - 1)\n    }\n    pub fn modulus() -> u32\
    \ {\n        M::M\n    }\n}\n\nmacro_rules! impl_from_int {\n    ($(($t:ty: $via:ty)),*)\
    \ => { $(\n        impl<M: Mod> From<$t> for Mint<M> {\n            fn from(x:\
    \ $t) -> Self {\n                Self::from_val((x as $via).rem_euclid(M::M as\
    \ $via) as u32)\n            }\n        }\n    )* };\n}\n\nimpl_from_int! {\n\
    \    (i8: i32), (i16: i32), (i32: i32), (i64: i64), (isize: isize),\n    (u8:\
    \ u32), (u16: u32), (u32: u32), (u64: u64), (usize: usize)\n}\n\nimpl<M: Mod,\
    \ T: Into<Mint<M>>> ops::Add<T> for Mint<M> {\n    type Output = Self;\n    fn\
    \ add(mut self, rhs: T) -> Self {\n        self += rhs;\n        self\n    }\n\
    }\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::AddAssign<T> for Mint<M> {\n    fn add_assign(&mut\
    \ self, rhs: T) {\n        self.val += rhs.into().val;\n        if self.val >=\
    \ M::M {\n            self.val -= M::M;\n        }\n    }\n}\n\nimpl<M: Mod> ops::Neg\
    \ for Mint<M> {\n    type Output = Self;\n    fn neg(self) -> Self {\n       \
    \ Mint::from_val(match self.val {\n            0 => 0,\n            s => M::M\
    \ - s,\n        })\n    }\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::Sub<T> for\
    \ Mint<M> {\n    type Output = Self;\n    fn sub(mut self, rhs: T) -> Self {\n\
    \        self -= rhs;\n        self\n    }\n}\n\nimpl<M: Mod, T: Into<Mint<M>>>\
    \ ops::SubAssign<T> for Mint<M> {\n    fn sub_assign(&mut self, rhs: T) {\n  \
    \      let rhs = rhs.into();\n        if self.val < rhs.val {\n            self.val\
    \ += M::M;\n        }\n        self.val -= rhs.val;\n    }\n}\n\nimpl<M: Mod,\
    \ T: Into<Mint<M>>> ops::Mul<T> for Mint<M> {\n    type Output = Self;\n    fn\
    \ mul(self, rhs: T) -> Self {\n        let val = self.val as u64 * rhs.into().val\
    \ as u64 % M::M as u64;\n        Self::from_val(val as u32)\n    }\n}\n\nimpl<M:\
    \ Mod, T: Into<Mint<M>>> ops::MulAssign<T> for Mint<M> {\n    fn mul_assign(&mut\
    \ self, rhs: T) {\n        *self = *self * rhs;\n    }\n}\n\nimpl<M: Mod, T: Into<Mint<M>>>\
    \ ops::Div<T> for Mint<M> {\n    type Output = Self;\n    fn div(mut self, rhs:\
    \ T) -> Self {\n        self /= rhs;\n        self\n    }\n}\n\nimpl<M: Mod, T:\
    \ Into<Mint<M>>> ops::DivAssign<T> for Mint<M> {\n    fn div_assign(&mut self,\
    \ rhs: T) {\n        *self *= rhs.into().pow(M::PHI - 1);\n    }\n}\n\nimpl<M:\
    \ Mod> iter::Sum for Mint<M> {\n    fn sum<I: Iterator<Item = Self>>(iter: I)\
    \ -> Self {\n        iter.fold(Self::from_val(0), |b, x| b + x)\n    }\n}\n\n\
    impl<M: Mod> iter::Product for Mint<M> {\n    fn product<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self {\n        iter.fold(Self::from_val(1), |b, x| b * x)\n    }\n}\n\
    \nimpl<M: Mod> fmt::Debug for Mint<M> {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result {\n        self.val.fmt(f)\n    }\n}\n\nimpl<M: Mod> fmt::Display\
    \ for Mint<M> {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n\
    \        self.val.fmt(f)\n    }\n}\n\nimpl<M: Mod> ZeroOne for Mint<M> {\n   \
    \ const ZERO: Self = Self { val: 0, _m: PhantomData };\n    const ONE: Self =\
    \ Self { val: 1, _m: PhantomData };\n}\n\n// impl<M: Mod> Print for Mint<M> {\n\
    //     fn print(w: &mut IO, x: Self) {\n//         w.print(x.value());\n//   \
    \  }\n// }\n\n// impl<M: Mod> Scan for Mint<M> {\n//     fn scan(io: &mut IO)\
    \ -> Self {\n//         io.scan::<u32>().into()\n//     }\n// }\n"
  dependsOn:
  - src/zo.rs
  isVerificationFile: false
  path: src/mint.rs
  requiredBy:
  - src/mint/num.rs
  - src/mint/conv.rs
  - src/tests.rs
  timestamp: '2021-02-08 00:55:36+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/mint.rs
layout: document
redirect_from:
- /library/src/mint.rs
- /library/src/mint.rs.html
title: src/mint.rs
---
