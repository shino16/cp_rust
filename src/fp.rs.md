---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_garner_test.rs
    title: test/src/bin/ntt_garner_test.rs
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
    \ _;\nuse crate::num::*;\nuse std::marker::PhantomData;\nuse std::{cmp, fmt, iter,\
    \ ops};\n\npub mod conv;\n\npub trait Mod: Default + Clone + Copy + PartialEq\
    \ + Eq {\n    const P: u32;\n    const K: u32; // -1 / P mod 2^32\n    const R2:\
    \ u32; // 2^64 mod P\n}\n\n// montgomery reduction (x -> x / 2^32 mod P)\nfn reduce<M:\
    \ Mod>(x: u64) -> u32 {\n    let s = M::K.wrapping_mul(x as u32);\n    ((x + s\
    \ as u64 * M::P as u64) >> 32) as u32\n}\n\nmacro_rules! def_mod {\n    ($name:ident,\
    \ $modu:expr, $k:expr) => {\n        #[derive(Default, Clone, Copy, PartialEq,\
    \ Eq, Debug)]\n        pub struct $name;\n        impl Mod for $name {\n     \
    \       const P: u32 = $modu;\n            const K: u32 = $k;\n            const\
    \ R2: u32 = ((1_u128 << 64) % $modu) as u32;\n        }\n    };\n}\n\ndef_mod!(ModA,\
    \ 1_000_000_007, 2_226_617_417);\ndef_mod!(ModB, 998_244_353, 998_244_351);\n\
    def_mod!(ModC, 1_012_924_417, 1_012_924_415);\ndef_mod!(ModD, 924_844_033, 924_844_031);\n\
    \n// modular arithmetics\n#[repr(transparent)]\n#[derive(Default, Clone, Copy)]\n\
    pub struct Fp<M: Mod> {\n    val: u32,\n    _m: PhantomData<M>,\n}\n\npub type\
    \ FpA = Fp<ModA>;\npub type FpB = Fp<ModB>;\npub type FpC = Fp<ModC>;\npub type\
    \ FpD = Fp<ModD>;\n\n/// mod 1_000_000_007\npub type F17 = FpA;\n/// mod 998_244_353\n\
    pub type F99 = FpB;\n\nimpl<M: Mod> Fp<M> {\n    pub const P: u32 = M::P;\n  \
    \  pub fn new(val: u32) -> Self { Fp::from_raw(reduce::<M>(val as u64 * M::R2\
    \ as u64)) }\n    fn from_raw(val: u32) -> Self { Fp { val, _m: PhantomData }\
    \ }\n    pub fn value(self) -> u32 {\n        let v = reduce::<M>(self.val as\
    \ u64);\n        if v >= M::P { v - M::P } else { v }\n    }\n    pub fn grow(self)\
    \ -> FpGrow<M> { FpGrow::from_raw((self.val as u64) << 32) }\n    pub fn mul_unreduced<T:\
    \ Into<Self>>(self, rhs: T) -> FpGrow<M> {\n        FpGrow::from_raw(self.val\
    \ as u64 * rhs.into().val as u64)\n    }\n    pub fn pow<I: Int>(self, k: I) ->\
    \ Self {\n        if self.val == 0 && k.is_zero() {\n            return Self::new(1);\n\
    \        }\n        let (mut e, mut k) = (self, k.rem_euclid((M::P - 1).as_()));\n\
    \        let mut res = Self::ONE;\n        while !k.is_zero() {\n            if\
    \ !(k & 1.as_()).is_zero() {\n                res *= e;\n            }\n     \
    \       e *= e;\n            k >>= 1;\n        }\n        res\n    }\n    pub\
    \ fn inv(self) -> Self {\n        let (mut a, mut b, mut u, mut v) = (M::P as\
    \ i32, self.value() as i32, 0, 1);\n        while b != 0 {\n            let t\
    \ = a / b;\n            a -= t * b;\n            u -= t * v;\n            std::mem::swap(&mut\
    \ a, &mut b);\n            std::mem::swap(&mut u, &mut v);\n        }\n      \
    \  debug_assert_eq!(a, 1);\n        if u < 0 {\n            u += M::P as i32;\n\
    \        }\n        Self::new(u as u32)\n    }\n}\n\n#[derive(Default, Clone,\
    \ Copy, PartialEq, Eq)]\npub struct FpGrow<M: Mod> {\n    val: u64,\n    _m: PhantomData<M>,\n\
    }\n\nimpl<M: Mod> FpGrow<M> {\n    const MOD: u64 = (M::P as u64) << 32;\n   \
    \ fn from_raw(val: u64) -> Self { Self { val, _m: PhantomData } }\n    pub fn\
    \ reduce(self) -> Fp<M> { Fp::from_raw(reduce::<M>(self.val)) }\n    pub fn value(self)\
    \ -> u32 { self.reduce().value() }\n}\n\nimpl<M: Mod> ops::Add<Self> for FpGrow<M>\
    \ {\n    type Output = Self;\n    fn add(mut self, rhs: Self) -> Self { self +=\
    \ rhs; self }\n}\n\nimpl<M: Mod> ops::AddAssign<Self> for FpGrow<M> {\n    fn\
    \ add_assign(&mut self, rhs: Self) {\n        self.val += rhs.val;\n        if\
    \ self.val >= Self::MOD * 2 { self.val -= Self::MOD * 2; }\n    }\n}\n\nimpl<M:\
    \ Mod> From<FpGrow<M>> for Fp<M> {\n    fn from(v: FpGrow<M>) -> Self { v.reduce()\
    \ }\n}\n\nimpl<M: Mod, I: Int> From<I> for Fp<M> {\n    fn from(x: I) -> Self\
    \ { Self::new(x.rem_euclid(M::P.as_()).as_()) }\n}\n\nimpl<M: Mod> cmp::PartialEq\
    \ for Fp<M> {\n    fn eq(&self, other: &Self) -> bool {\n        let val = |obj:\
    \ &Fp<M>| if obj.val >= M::P  { obj.val - M::P } else { obj.val };\n        val(self)\
    \ == val(other)\n    }\n}\n\nimpl<M: Mod> cmp::Eq for Fp<M> {}\n\nimpl<M: Mod,\
    \ T: Into<Fp<M>>> ops::AddAssign<T> for Fp<M> {\n    fn add_assign(&mut self,\
    \ rhs: T) {\n        self.val += rhs.into().val;\n        if self.val >= M::P\
    \ * 2 { self.val -= M::P * 2; }\n    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::SubAssign<T>\
    \ for Fp<M> {\n    fn sub_assign(&mut self, rhs: T) {\n        let rhs = rhs.into();\n\
    \        if self.val < rhs.val { self.val += M::P * 2; }\n        self.val -=\
    \ rhs.val;\n    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::MulAssign<T> for Fp<M>\
    \ {\n    fn mul_assign(&mut self, rhs: T) { *self = self.mul_unreduced(rhs).reduce();\
    \ }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T> for Fp<M> {\n    fn div_assign(&mut\
    \ self, rhs: T) { *self *= rhs.into().inv(); }\n}\n\nmacro_rules! impl_binop {\n\
    \    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(\n\
    \        impl<M: Mod, T: Into<Fp<M>>> ops::$Op<T> for Fp<M> {\n            type\
    \ Output = Self;\n            fn $op(mut self, rhs: T) -> Self {\n           \
    \     ops::$OpAssign::$op_assign(&mut self, rhs); self\n            }\n      \
    \  }\n    )* };\n}\n\nimpl_binop!((Add, add, AddAssign, add_assign), (Sub, sub,\
    \ SubAssign, sub_assign),\n            (Mul, mul, MulAssign, mul_assign), (Div,\
    \ div, DivAssign, div_assign));\n\nimpl<M: Mod> ops::Neg for Fp<M> {\n    type\
    \ Output = Self;\n    fn neg(self) -> Self { Fp::from_raw(M::P * 2 - self.val)\
    \ }\n}\n\nimpl<M: Mod> iter::Sum for Fp<M> {\n    fn sum<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self { iter.fold(Self::ZERO, |b, x| b + x) }\n}\n\nimpl<M: Mod> iter::Product\
    \ for Fp<M> {\n    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ONE,\
    \ |b, x| b * x) }\n}\n\nimpl<M: Mod> fmt::Debug for Fp<M> {\n    fn fmt(&self,\
    \ f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f) }\n}\n\nimpl<M:\
    \ Mod> fmt::Display for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter) ->\
    \ fmt::Result { self.value().fmt(f) }\n}\n\nimpl<M: Mod> ZeroOne for Fp<M> {\n\
    \    const ZERO: Self = Self { val: 0, _m: PhantomData };\n    const ONE: Self\
    \ = Self {\n        val: M::P.wrapping_neg() % M::P,\n        _m: PhantomData,\n\
    \    };\n}\n\nimpl<M: Mod> Num for Fp<M> {}\n\nimpl<M: Mod> Print for Fp<M> {\n\
    \    fn print(w: &mut IO, x: Self) { w.print(x.value()); }\n}\n\nimpl<M: Mod>\
    \ Scan for Fp<M> {\n    fn scan(io: &mut IO) -> Self { Self::new(io.scan()) }\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/fp.rs
  requiredBy: []
  timestamp: '2020-11-28 19:21:23+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/cargo_test.rs
documentation_of: src/fp.rs
layout: document
redirect_from:
- /library/src/fp.rs
- /library/src/fp.rs.html
title: src/fp.rs
---
