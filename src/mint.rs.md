---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':heavy_check_mark:'
    path: src/mint/io.rs
    title: src/mint/io.rs
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
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/mint.rs\n"
  code: "pub use crate::def_mod;\npub use crate::zo::ZeroOne;\nuse std::marker::PhantomData;\n\
    use std::str::FromStr;\nuse std::{fmt, iter, ops};\n\npub mod conv;\npub mod io;\n\
    \npub trait Mod: Default + Clone + Copy + PartialEq + Eq {\n    const M: u32;\n\
    \    const PHI: u32;\n}\n\n#[macro_export]\nmacro_rules! def_mod {\n    ($name:ident,\
    \ $modulus:expr) => {\n        def_mod!($name, $modulus, $modulus - 1);\n    };\n\
    \    ($name:ident, $modulus:expr, $phi:expr) => {\n        #[derive(Default, Clone,\
    \ Copy, PartialEq, Eq, Debug)]\n        pub struct $name;\n        impl Mod for\
    \ $name {\n            const M: u32 = $modulus;\n            const PHI: u32 =\
    \ $phi;\n        }\n    };\n}\n\ndef_mod!(ModA, 1_000_000_007);\ndef_mod!(ModB,\
    \ 998_244_353);\ndef_mod!(ModC, 1_012_924_417);\ndef_mod!(ModD, 924_844_033);\n\
    \n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct Mint<M: Mod> {\n\
    \    pub val: u32,\n    _m: PhantomData<M>,\n}\n\npub type MintA = Mint<ModA>;\n\
    pub type MintB = Mint<ModB>;\npub type MintC = Mint<ModC>;\npub type MintD = Mint<ModD>;\n\
    pub type Mint17 = MintA;\npub type Mint99 = MintB;\n\nimpl<M: Mod> Mint<M> {\n\
    \    pub const M: u32 = M::M;\n    pub fn new(val: i64) -> Self { Self::from_val(val.rem_euclid(M::M\
    \ as i64) as u32) }\n    pub fn from_val(val: u32) -> Self { Mint { val, _m: PhantomData\
    \ } }\n    pub fn value(self) -> u32 { self.val }\n    pub fn pow(self, mut exp:\
    \ u64) -> Self {\n        if self.val == 0 && exp == 0 {\n            return Self::from_val(1);\n\
    \        }\n        let mut b = self;\n        let mut res = Self::from_val(1);\n\
    \        while exp != 0 {\n            if exp % 2 == 1 {\n                res\
    \ *= b;\n            }\n            b *= b;\n            exp >>= 1;\n        }\n\
    \        res\n    }\n    pub fn inv(self) -> Self { self.pow(M::PHI as u64 - 1)\
    \ }\n    pub fn modulus() -> u32 { M::M }\n}\n\nmacro_rules! impl_from_int {\n\
    \    ($(($t:ty: $via:ty)),* $(,)?) => { $(\n        impl<M: Mod> From<$t> for\
    \ Mint<M> {\n            fn from(x: $t) -> Self { Self::from_val((x as $via).rem_euclid(M::M\
    \ as $via) as u32) }\n        }\n    )* };\n}\nimpl_from_int! {\n    (i8: i32),\
    \ (i16: i32), (i32: i32), (i64: i64), (isize: i64),\n    (u8: u32), (u16: u32),\
    \ (u32: u32), (u64: u64), (usize: u64),\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::Add<T>\
    \ for Mint<M> {\n    type Output = Self;\n    fn add(mut self, rhs: T) -> Self\
    \ {\n        self += rhs;\n        self\n    }\n}\nimpl<M: Mod, T: Into<Mint<M>>>\
    \ ops::AddAssign<T> for Mint<M> {\n    fn add_assign(&mut self, rhs: T) {\n  \
    \      self.val += rhs.into().val;\n        if self.val >= M::M {\n          \
    \  self.val -= M::M;\n        }\n    }\n}\nimpl<M: Mod> ops::Neg for Mint<M> {\n\
    \    type Output = Self;\n    fn neg(self) -> Self {\n        Mint::from_val(if\
    \ self.val == 0 { 0 } else { M::M - self.val })\n    }\n}\nimpl<M: Mod, T: Into<Mint<M>>>\
    \ ops::Sub<T> for Mint<M> {\n    type Output = Self;\n    fn sub(mut self, rhs:\
    \ T) -> Self {\n        self -= rhs;\n        self\n    }\n}\nimpl<M: Mod, T:\
    \ Into<Mint<M>>> ops::SubAssign<T> for Mint<M> {\n    fn sub_assign(&mut self,\
    \ rhs: T) {\n        let rhs = rhs.into();\n        if self.val < rhs.val {\n\
    \            self.val += M::M;\n        }\n        self.val -= rhs.val;\n    }\n\
    }\nimpl<M: Mod, T: Into<Mint<M>>> ops::Mul<T> for Mint<M> {\n    type Output =\
    \ Self;\n    fn mul(self, rhs: T) -> Self {\n        let val = self.val as u64\
    \ * rhs.into().val as u64 % M::M as u64;\n        Self::from_val(val as u32)\n\
    \    }\n}\nimpl<M: Mod, T: Into<Mint<M>>> ops::MulAssign<T> for Mint<M> {\n  \
    \  fn mul_assign(&mut self, rhs: T) { *self = *self * rhs; }\n}\nimpl<M: Mod,\
    \ T: Into<Mint<M>>> ops::Div<T> for Mint<M> {\n    type Output = Self;\n    fn\
    \ div(mut self, rhs: T) -> Self {\n        self /= rhs;\n        self\n    }\n\
    }\nimpl<M: Mod, T: Into<Mint<M>>> ops::DivAssign<T> for Mint<M> {\n    fn div_assign(&mut\
    \ self, rhs: T) { *self *= rhs.into().pow(M::PHI as u64 - 1); }\n}\nimpl<M: Mod>\
    \ iter::Sum for Mint<M> {\n    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self\
    \ {\n        iter.fold(Self::from_val(0), |b, x| b + x)\n    }\n}\nimpl<M: Mod>\
    \ iter::Product for Mint<M> {\n    fn product<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self {\n        iter.fold(Self::from_val(1), |b, x| b * x)\n    }\n}\n\
    impl<M: Mod> fmt::Debug for Mint<M> {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result { self.val.fmt(f) }\n}\nimpl<M: Mod> fmt::Display for Mint<M>\
    \ {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.val.fmt(f)\
    \ }\n}\nimpl<M: Mod> FromStr for Mint<M> {\n    type Err = <u32 as FromStr>::Err;\n\
    \    fn from_str(s: &str) -> Result<Self, Self::Err> { u32::from_str(s).map(Self::from)\
    \ }\n}\nimpl<M: Mod> ZeroOne for Mint<M> {\n    const ZERO: Self = Self { val:\
    \ 0, _m: PhantomData };\n    const ONE: Self = Self { val: 1, _m: PhantomData\
    \ };\n}\n"
  dependsOn:
  - src/lib.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/mint.rs
  requiredBy:
  - src/tests.rs
  - src/mint/conv.rs
  - src/mint/io.rs
  timestamp: '2021-03-22 00:48:45+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/mint.rs
layout: document
redirect_from:
- /library/src/mint.rs
- /library/src/mint.rs.html
title: src/mint.rs
---
