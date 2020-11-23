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
  - icon: ':x:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use crate::as_int::*;\nuse crate::io::*;\npub use crate::num::ZeroOne as\
    \ _;\nuse crate::num::*;\nuse std::marker::PhantomData;\nuse std::{fmt, iter,\
    \ ops};\n\npub mod conv;\n\npub trait Mod: Default + Clone + Copy + PartialEq\
    \ + Eq {\n    const P: u32;\n    const K: u32; // -1 / P mod 2^32\n    const R2:\
    \ u32; // 2^64 mod P\n}\n\n// montgomery reduction (x -> x / 2^32 mod P)\nfn reduce<M:\
    \ Mod>(x: u64) -> u32 {\n    let s = M::K.wrapping_mul(x as u32);\n    let t =\
    \ x + s as u64 * M::P as u64;\n    let u = (t >> 32) as u32;\n    if u < M::P\
    \ { u } else { u - M::P }\n}\n\nmacro_rules! def_mod {\n    ($name:ident, $modu:expr,\
    \ $k:expr) => {\n        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n\
    \        pub struct $name;\n        impl Mod for $name {\n            const P:\
    \ u32 = $modu;\n            const K: u32 = $k;\n            const R2: u32 = ((1_u128\
    \ << 64) % $modu) as u32;\n        }\n    };\n}\n\ndef_mod!(ModA, 1_000_000_007,\
    \ 2_226_617_417);\ndef_mod!(ModB, 998_244_353, 998_244_351);\ndef_mod!(ModC, 1_012_924_417,\
    \ 1_012_924_415);\ndef_mod!(ModD, 924_844_033, 924_844_031);\n\n// modular arithmetics\n\
    #[repr(transparent)]\n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct\
    \ Fp<M: Mod> {\n    val: u32,\n    _m: PhantomData<M>,\n}\n\npub type FpA = Fp<ModA>;\n\
    pub type FpB = Fp<ModB>;\npub type FpC = Fp<ModC>;\npub type FpD = Fp<ModD>;\n\
    \n/// mod 1_000_000_007\npub type F17 = FpA;\n/// mod 998_244_353\npub type F99\
    \ = FpB;\n\nimpl<M: Mod> Fp<M> {\n    pub const P: u32 = M::P;\n    pub fn new(val:\
    \ u32) -> Self { Fp::from_raw(reduce::<M>(val as u64 * M::R2 as u64)) }\n    fn\
    \ from_raw(val: u32) -> Self { Fp { val, _m: PhantomData } }\n    pub fn value(self)\
    \ -> u32 { reduce::<M>(self.val as u64) }\n    pub fn grow(self) -> FpGrow<M>\
    \ { FpGrow::from_raw((self.val as u64) << 32) }\n    pub fn mul_unreduced<T: Into<Self>>(self,\
    \ rhs: T) -> FpGrow<M> {\n        FpGrow::from_raw(self.val as u64 * rhs.into().val\
    \ as u64)\n    }\n    pub fn pow<I: Int>(self, k: I) -> Self {\n        if self.val\
    \ == 0 && k.is_zero() { return Self::new(1); }\n        let (mut e, mut k) = (self,\
    \ k.rem_euclid((M::P - 1).as_()));\n        let mut res = Self::ONE;\n       \
    \ while !k.is_zero() {\n            if !(k & 1.as_()).is_zero() { res *= e; }\n\
    \            e *= e; k >>= 1;\n        }\n        res\n    }\n    pub fn inv(self)\
    \ -> Self {\n        let (mut a, mut b, mut u, mut v) = (M::P as i32, self.value()\
    \ as i32, 0, 1);\n        while b != 0 {\n            let t = a / b;\n       \
    \     a -= t * b; u -= t * v;\n            std::mem::swap(&mut a, &mut b);\n \
    \           std::mem::swap(&mut u, &mut v);\n        }\n        debug_assert_eq!(a,\
    \ 1);\n        if u < 0 { u += M::P as i32; }\n        Self::new(u as u32)\n \
    \   }\n}\n\n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct FpGrow<M:\
    \ Mod> {\n    val: u64,\n    _m: PhantomData<M>,\n}\n\nimpl<M: Mod> FpGrow<M>\
    \ {\n    fn from_raw(val: u64) -> Self { Self { val, _m: PhantomData } }\n   \
    \ pub fn reduce(self) -> Fp<M> { Fp::from_raw(reduce::<M>(self.val)) }\n    pub\
    \ fn value(self) -> u32 { self.reduce().value() }\n}\n\nimpl<M: Mod> ops::Add<Self>\
    \ for FpGrow<M> {\n    type Output = Self;\n    fn add(mut self, rhs: Self) ->\
    \ Self { self += rhs; self }\n}\n\nimpl<M: Mod> ops::AddAssign<Self> for FpGrow<M>\
    \ {\n    fn add_assign(&mut self, rhs: Self) { self.val += rhs.val; }\n}\n\nimpl<M:\
    \ Mod> From<FpGrow<M>> for Fp<M> {\n    fn from(v: FpGrow<M>) -> Self { v.reduce()\
    \ }\n}\n\nimpl<M: Mod, I: Int> From<I> for Fp<M> {\n    fn from(x: I) -> Self\
    \ { Self::new(x.rem_euclid(M::P.as_()).as_()) }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>>\
    \ ops::Add<T> for Fp<M> {\n    type Output = Self;\n    fn add(mut self, rhs:\
    \ T) -> Self { self += rhs; self }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::AddAssign<T>\
    \ for Fp<M> {\n    fn add_assign(&mut self, rhs: T) {\n        self.val += rhs.into().val;\n\
    \        if self.val >= M::P { self.val -= M::P; }\n    }\n}\n\nimpl<M: Mod> ops::Neg\
    \ for Fp<M> {\n    type Output = Self;\n    fn neg(self) -> Self { Fp::from_raw(M::P\
    \ - self.val) }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::Sub<T> for Fp<M> {\n \
    \   type Output = Self;\n    fn sub(mut self, rhs: T) -> Self { self -= rhs; self\
    \ }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::SubAssign<T> for Fp<M> {\n    fn sub_assign(&mut\
    \ self, rhs: T) {\n        let rhs = rhs.into();\n        if self.val < rhs.val\
    \ { self.val += M::P; }\n        self.val -= rhs.val;\n    }\n}\n\nimpl<M: Mod,\
    \ T: Into<Fp<M>>> ops::Mul<T> for Fp<M> {\n    type Output = Self;\n    fn mul(self,\
    \ rhs: T) -> Self { self.mul_unreduced(rhs).reduce() }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>>\
    \ ops::MulAssign<T> for Fp<M> {\n    fn mul_assign(&mut self, rhs: T) { *self\
    \ = *self * rhs; }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::Div<T> for Fp<M> {\n\
    \    type Output = Self;\n    fn div(mut self, rhs: T) -> Self { self /= rhs;\
    \ self }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T> for Fp<M> {\n  \
    \  fn div_assign(&mut self, rhs: T) { *self *= rhs.into().inv(); }\n}\n\nimpl<M:\
    \ Mod> iter::Sum for Fp<M> {\n    fn sum<I: Iterator<Item = Self>>(iter: I) ->\
    \ Self {\n        iter.fold(Self::ZERO, |b, x| b + x)\n    }\n}\n\nimpl<M: Mod>\
    \ iter::Product for Fp<M> {\n    fn product<I: Iterator<Item = Self>>(iter: I)\
    \ -> Self {\n        iter.fold(Self::ONE, |b, x| b * x)\n    }\n}\n\nimpl<M: Mod>\
    \ fmt::Debug for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result\
    \ { self.value().fmt(f) }\n}\n\nimpl<M: Mod> fmt::Display for Fp<M> {\n    fn\
    \ fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f) }\n\
    }\n\nimpl<M: Mod> ZeroOne for Fp<M> {\n    const ZERO: Self = Self { val: 0, _m:\
    \ PhantomData };\n    const ONE: Self = Self { val: M::P.wrapping_neg() % M::P,\
    \ _m: PhantomData };\n}\n\nimpl<M: Mod> Num for Fp<M> {}\n\nimpl<M: Mod> Print\
    \ for Fp<M> {\n    fn print(w: &mut IO, x: Self) { w.print(x.value()); }\n}\n\n\
    impl<M: Mod> Scan for Fp<M> {\n    fn scan(io: &mut IO) -> Self { Self::new(io.scan())\
    \ }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/fp.rs
  requiredBy: []
  timestamp: '2020-11-24 01:55:32+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/fp.rs
layout: document
redirect_from:
- /library/src/fp.rs
- /library/src/fp.rs.html
title: src/fp.rs
---
