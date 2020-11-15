---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/lib.rs
    title: src/lib.rs
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
  code: "use crate::as_int::*;\nuse crate::io::*;\npub use crate::num::ZeroOne as\
    \ _;\nuse crate::num::*;\nuse std::marker::PhantomData;\nuse std::{fmt, iter,\
    \ ops};\n\npub trait Mod: Default + Clone + Copy + PartialEq + Eq {\n    const\
    \ P: u32;\n    const K: u32; // -1 / P mod 2^32\n    const R2: u32; // 2^64 mod\
    \ P\n}\n\n// montgomery reduction (x -> x / 2^32 mod P)\nfn redc<M: Mod>(x: u64)\
    \ -> u32 {\n    let s = M::K.wrapping_mul(x as u32);\n    let t = x + s as u64\
    \ * M::P as u64;\n    let u = (t >> 32) as u32;\n    if u < M::P { u } else {\
    \ u - M::P }\n}\n\nmacro_rules! def_mod {\n    ($name:ident, $modu:expr, $k:expr,\
    \ $r2:expr) => {\n        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n\
    \        pub struct $name;\n        impl Mod for $name {\n            const P:\
    \ u32 = $modu;\n            const K: u32 = $k;\n            const R2: u32 = $r2;\n\
    \        }\n    };\n}\n\ndef_mod!(Mod17, 1_000_000_007, 2_226_617_417, 582_344_008);\n\
    def_mod!(Mod99, 998_244_353, 998_244_351, 932_051_910);\ndef_mod!(Mod10, 1_012_924_417,\
    \ 1_012_924_415, 818_184_550);\ndef_mod!(Mod92, 924_844_033, 924_844_031, 404_973_864);\n\
    \n// modular arithmetics\n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub\
    \ struct Fp<M: Mod> {\n    val: u32,\n    _m: PhantomData<M>,\n}\n\npub type Fp17\
    \ = Fp<Mod17>;\npub type Fp99 = Fp<Mod99>;\npub type Fp10 = Fp<Mod10>;\npub type\
    \ Fp92 = Fp<Mod92>;\n\nimpl<M: Mod> Fp<M> {\n    pub fn new(val: u32) -> Self\
    \ {\n        Fp::from_raw(redc::<M>(val as u64 * M::R2 as u64))\n    }\n    fn\
    \ from_raw(val: u32) -> Self {\n        Fp { val, _m: PhantomData }\n    }\n \
    \   pub fn value(self) -> u32 {\n        redc::<M>(self.val as u64)\n    }\n \
    \   pub fn grow(self) -> FpGrow<M> {\n        FpGrow::from_raw((self.val as u64)\
    \ << 32)\n    }\n    pub fn mul_unreduced<T: Into<Self>>(self, rhs: T) -> FpGrow<M>\
    \ {\n        FpGrow::from_raw(self.val as u64 * rhs.into().val as u64)\n    }\n\
    \    pub fn pow<I: Int>(self, k: I) -> Self {\n        if self.val == 0 && k.is_zero()\
    \ {\n            return Self::new(1);\n        }\n        let (mut e, mut k) =\
    \ (self, k.rem_euclid((M::P - 1).as_()));\n        let mut res = Self::ONE;\n\
    \        while !k.is_zero() {\n            if !(k % 2.as_()).is_zero() {\n   \
    \             res *= e;\n            }\n            e *= e;\n            k >>=\
    \ 1;\n        }\n        res\n    }\n    pub fn inv(self) -> Self {\n        let\
    \ (mut a, mut b, mut u, mut v) = (self.value() as i32, M::P as i32, 1, 0);\n \
    \       while b != 0 {\n            let t = a / b;\n            a -= t * b;\n\
    \            u -= t * v;\n            std::mem::swap(&mut a, &mut b);\n      \
    \      std::mem::swap(&mut u, &mut v);\n        }\n        if u < 0 {\n      \
    \      u += M::P as i32;\n        }\n        Self::new(u as u32)\n    }\n}\n\n\
    #[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct FpGrow<M: Mod> {\n\
    \    val: u64,\n    _m: PhantomData<M>,\n}\n\nimpl<M: Mod> FpGrow<M> {\n    fn\
    \ from_raw(val: u64) -> Self {\n        Self { val, _m: PhantomData }\n    }\n\
    \    pub fn reduce(self) -> Fp<M> {\n        Fp::from_raw(redc::<M>(self.val))\n\
    \    }\n    pub fn value(self) -> u32 {\n        self.reduce().value()\n    }\n\
    }\n\nimpl<M: Mod> From<FpGrow<M>> for Fp<M> {\n    fn from(v: FpGrow<M>) -> Self\
    \ {\n        v.reduce()\n    }\n}\n\nimpl<M: Mod, I: Int> From<I> for Fp<M> {\n\
    \    fn from(x: I) -> Self {\n        Self::new(x.rem_euclid(M::P.as_()).as_())\n\
    \    }\n}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::Add<T> for Fp<M> {\n    type Output\
    \ = Self;\n    fn add(mut self, rhs: T) -> Self {\n        self += rhs;\n    \
    \    self\n    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::AddAssign<T> for Fp<M>\
    \ {\n    fn add_assign(&mut self, rhs: T) {\n        self.val += rhs.into().val;\n\
    \        if self.val >= M::P {\n            self.val -= M::P;\n        }\n   \
    \ }\n}\nimpl<M: Mod> ops::Neg for Fp<M> {\n    type Output = Self;\n    fn neg(self)\
    \ -> Self {\n        Fp::from_raw(M::P - self.val)\n    }\n}\nimpl<M: Mod, T:\
    \ Into<Fp<M>>> ops::Sub<T> for Fp<M> {\n    type Output = Self;\n    fn sub(mut\
    \ self, rhs: T) -> Self {\n        self -= rhs;\n        self\n    }\n}\nimpl<M:\
    \ Mod, T: Into<Fp<M>>> ops::SubAssign<T> for Fp<M> {\n    fn sub_assign(&mut self,\
    \ rhs: T) {\n        let rhs = rhs.into();\n        if self.val < rhs.val {\n\
    \            self.val += M::P;\n        }\n        self.val -= rhs.val;\n    }\n\
    }\nimpl<M: Mod, T: Into<Fp<M>>> ops::Mul<T> for Fp<M> {\n    type Output = Self;\n\
    \    fn mul(self, rhs: T) -> Self {\n        self.mul_unreduced(rhs).reduce()\n\
    \    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::MulAssign<T> for Fp<M> {\n    fn\
    \ mul_assign(&mut self, rhs: T) {\n        *self = *self * rhs;\n    }\n}\nimpl<M:\
    \ Mod, T: Into<Fp<M>>> ops::Div<T> for Fp<M> {\n    type Output = Self;\n    fn\
    \ div(mut self, rhs: T) -> Self {\n        self /= rhs;\n        self\n    }\n\
    }\nimpl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T> for Fp<M> {\n    fn div_assign(&mut\
    \ self, rhs: T) {\n        *self *= rhs.into().inv();\n    }\n}\n\nimpl<M: Mod>\
    \ iter::Sum for Fp<M> {\n    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self\
    \ {\n        iter.fold(Self::from_raw(0), |b, x| b + x)\n    }\n}\nimpl<M: Mod>\
    \ iter::Product for Fp<M> {\n    fn product<I: Iterator<Item = Self>>(iter: I)\
    \ -> Self {\n        iter.fold(Self::from_raw(1), |b, x| b * x)\n    }\n}\n\n\
    impl<M: Mod> fmt::Debug for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result {\n        self.value().fmt(f)\n    }\n}\nimpl<M: Mod> fmt::Display\
    \ for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n  \
    \      self.value().fmt(f)\n    }\n}\n\nimpl<M: Mod> ZeroOne for Fp<M> {\n   \
    \ const ZERO: Self = Self { val: 0, _m: PhantomData };\n    const ONE: Self =\
    \ Self {\n        val: M::P.wrapping_neg() % M::P,\n        _m: PhantomData,\n\
    \    };\n}\n\nimpl<M: Mod> Num for Fp<M> {}\n\nimpl<M: Mod> Print for Fp<M> {\n\
    \    fn print(w: &mut IO, x: Self) {\n        w.print(x.value());\n    }\n}\n\
    impl<M: Mod> Scan for Fp<M> {\n    fn scan(io: &mut IO) -> Self {\n        Self::new(io.scan())\n\
    \    }\n}\n\nimpl<M: Mod> ops::Add<Self> for FpGrow<M> {\n    type Output = Self;\n\
    \    fn add(mut self, rhs: Self) -> Self {\n        self += rhs;\n        self\n\
    \    }\n}\nimpl<M: Mod> ops::AddAssign<Self> for FpGrow<M> {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        self.val += rhs.val;\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/fp.rs
  requiredBy:
  - src/lib.rs
  timestamp: '2020-11-04 20:37:29+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_test.rs
documentation_of: src/fp.rs
layout: document
redirect_from:
- /library/src/fp.rs
- /library/src/fp.rs.html
title: src/fp.rs
---
