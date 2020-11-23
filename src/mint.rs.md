---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':x:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::io::*;\npub use crate::num::ZeroOne as _;\nuse crate::num::*;\n\
    use std::marker::PhantomData;\nuse std::{fmt, iter, ops};\n\npub mod conv;\n\n\
    pub trait Mod: Default + Clone + Copy + PartialEq + Eq {\n    const P: u32;\n\
    \    const PHI: u32;\n}\n\nmacro_rules! def_mod {\n    ($name:ident, $modulus:expr)\
    \ => {\n        def_mod!($name, $modulus, $modulus - 1);\n    };\n    ($name:ident,\
    \ $modulus:expr, $phi:expr) => {\n        #[derive(Default, Clone, Copy, PartialEq,\
    \ Eq, Debug)]\n        pub struct $name;\n        impl Mod for $name {\n     \
    \       const P: u32 = $modulus;\n            const PHI: u32 = $phi;\n       \
    \ }\n    };\n}\n\ndef_mod!(ModA, 1_000_000_007);\ndef_mod!(ModB, 998_244_353);\n\
    def_mod!(ModC, 1_012_924_417);\ndef_mod!(ModD, 924_844_033);\n\n#[derive(Default,\
    \ Clone, Copy, PartialEq, Eq)]\npub struct Mint<P: Mod> {\n    pub val: u32,\n\
    \    _m: PhantomData<P>,\n}\n\npub type MintA = Mint<ModA>;\npub type MintB =\
    \ Mint<ModB>;\npub type MintC = Mint<ModC>;\npub type MintD = Mint<ModD>;\n\n\
    pub type Mint17 = MintA;\npub type Mint99 = MintB;\n\nimpl<P: Mod> Mint<P> {\n\
    \    const P: u32 = P::P;\n    pub fn new(val: i64) -> Self { Self::from_val(val.rem_euclid(P::P\
    \ as i64) as u32) }\n    pub fn from_val(val: u32) -> Self { Mint { val, _m: PhantomData\
    \ } }\n    pub fn value(self) -> u32 { self.val }\n    pub fn pow(self, mut exp:\
    \ u32) -> Self {\n        if self.val == 0 && exp == 0 {\n            return Self::from_val(1);\n\
    \        }\n        let mut e = self;\n        let mut res = Self::from_val(1);\n\
    \        while exp != 0 {\n            if exp % 2 == 1 {\n                res\
    \ *= e;\n            }\n            e *= e;\n            exp >>= 1;\n        }\n\
    \        res\n    }\n    pub fn inv(self) -> Self { self.pow(P::PHI - 1) }\n \
    \   pub fn modulus() -> u32 { P::P }\n}\n\nmacro_rules! impl_from_int {\n    ($(($ty:ty:\
    \ $via:ty)),*) => { $(\n        impl<P: Mod> From<$ty> for Mint<P> {\n       \
    \     fn from(x: $ty) -> Self {\n                Self::from_val((x as $via).rem_euclid(P::P\
    \ as $via) as u32)\n            }\n        }\n    )* };\n}\n\nimpl_from_int! {\n\
    \    (i8: i32), (i16: i32), (i32: i32), (i64: i64), (isize: isize),\n    (u8:\
    \ u32), (u16: u32), (u32: u32), (u64: u64), (usize: usize)\n}\n\nimpl<P: Mod,\
    \ T: Into<Mint<P>>> ops::Add<T> for Mint<P> {\n    type Output = Self;\n    fn\
    \ add(mut self, rhs: T) -> Self { self += rhs; self }\n}\n\nimpl<P: Mod, T: Into<Mint<P>>>\
    \ ops::AddAssign<T> for Mint<P> {\n    fn add_assign(&mut self, rhs: T) {\n  \
    \      self.val += rhs.into().val;\n        if self.val >= P::P {\n          \
    \  self.val -= P::P;\n        }\n    }\n}\n\nimpl<P: Mod> ops::Neg for Mint<P>\
    \ {\n    type Output = Self;\n    fn neg(self) -> Self {\n        Mint::from_val(match\
    \ self.val {\n            0 => 0,\n            s => P::P - s,\n        })\n  \
    \  }\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::Sub<T> for Mint<P> {\n    type\
    \ Output = Self;\n    fn sub(mut self, rhs: T) -> Self { self -= rhs; self }\n\
    }\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::SubAssign<T> for Mint<P> {\n    fn sub_assign(&mut\
    \ self, rhs: T) {\n        let rhs = rhs.into();\n        if self.val < rhs.val\
    \ {\n            self.val += P::P;\n        }\n        self.val -= rhs.val;\n\
    \    }\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::Mul<T> for Mint<P> {\n    type\
    \ Output = Self;\n    fn mul(self, rhs: T) -> Self {\n        let val = self.val\
    \ as u64 * rhs.into().val as u64 % P::P as u64;\n        Self::from_val(val as\
    \ u32)\n    }\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::MulAssign<T> for Mint<P>\
    \ {\n    fn mul_assign(&mut self, rhs: T) { *self = *self * rhs; }\n}\n\nimpl<P:\
    \ Mod, T: Into<Mint<P>>> ops::Div<T> for Mint<P> {\n    type Output = Self;\n\
    \    fn div(mut self, rhs: T) -> Self { self /= rhs; self }\n}\n\nimpl<P: Mod,\
    \ T: Into<Mint<P>>> ops::DivAssign<T> for Mint<P> {\n    fn div_assign(&mut self,\
    \ rhs: T) { *self *= rhs.into().pow(P::PHI - 1); }\n}\n\nimpl<P: Mod> iter::Sum\
    \ for Mint<P> {\n    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {\n   \
    \     iter.fold(Self::from_val(0), |b, x| b + x)\n    }\n}\n\nimpl<P: Mod> iter::Product\
    \ for Mint<P> {\n    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {\n\
    \        iter.fold(Self::from_val(1), |b, x| b * x)\n    }\n}\n\nimpl<P: Mod>\
    \ fmt::Debug for Mint<P> {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result\
    \ { self.val.fmt(f) }\n}\n\nimpl<P: Mod> fmt::Display for Mint<P> {\n    fn fmt(&self,\
    \ f: &mut fmt::Formatter) -> fmt::Result { self.val.fmt(f) }\n}\n\nimpl<P: Mod>\
    \ ZeroOne for Mint<P> {\n    const ZERO: Self = Self { val: 0, _m: PhantomData\
    \ };\n    const ONE: Self = Self { val: 1, _m: PhantomData };\n}\n\nimpl<P: Mod>\
    \ Num for Mint<P> {}\n\nimpl<M: Mod> Print for Mint<M> {\n    fn print(w: &mut\
    \ IO, x: Self) { w.print(x.value()); }\n}\n\nimpl<M: Mod> Scan for Mint<M> {\n\
    \    fn scan(io: &mut IO) -> Self { io.scan::<u32>().into() }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/mint.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/mint.rs
layout: document
redirect_from:
- /library/src/mint.rs
- /library/src/mint.rs.html
title: src/mint.rs
---
