---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bits.rs
    title: src/bits.rs
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/fpacc64.rs\n"
  code: "use crate::cast::*;\npub use crate::int::ZeroOne;\nuse crate::int::*;\nuse\
    \ crate::io::*;\nuse std::marker::PhantomData;\nuse std::{cmp, fmt, iter, ops};\n\
    \npub trait Mod: Default + Clone + Copy + PartialEq + Eq {\n    const P: u32;\n\
    \    const K: u32; // -1 / P mod 2^32\n    const R2: u32; // 2^64 mod P\n}\n\n\
    // montgomery reduction (x -> x / 2^32 mod P)\nfn reduce<M: Mod>(x: u64) -> u32\
    \ {\n    let s = M::K.wrapping_mul(x as u32);\n    ((x + s as u64 * M::P as u64)\
    \ >> 32) as u32\n}\n\nmacro_rules! def_mod {\n    ($name:ident, $modu:expr, $k:expr)\
    \ => {\n        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n      \
    \  pub struct $name;\n        impl Mod for $name {\n            const P: u32 =\
    \ $modu;\n            const K: u32 = $k;\n            const R2: u32 = ((1_u128\
    \ << 64) % $modu) as u32;\n        }\n    };\n}\n\ndef_mod!(ModA, 1_000_000_007,\
    \ 2_226_617_417);\ndef_mod!(ModB, 998_244_353, 998_244_351);\ndef_mod!(ModC, 1_012_924_417,\
    \ 1_012_924_415);\ndef_mod!(ModD, 924_844_033, 924_844_031);\n\n// modular arithmetics\n\
    #[repr(transparent)]\n#[derive(Default, Clone, Copy)]\npub struct Fp<M: Mod> {\n\
    \    val: u32,\n    _m: PhantomData<M>,\n}\n\npub type FpA = Fp<ModA>;\npub type\
    \ FpB = Fp<ModB>;\npub type FpC = Fp<ModC>;\npub type FpD = Fp<ModD>;\n\n/// mod\
    \ 1_000_000_007\npub type F17 = FpA;\n\n/// mod 998_244_353\npub type F99 = FpB;\n\
    \nimpl<M: Mod> Fp<M> {\n    pub const P: u32 = M::P;\n    pub fn new(val: u32)\
    \ -> Self {\n        Fp::from_raw(reduce::<M>(val as u64 * M::R2 as u64))\n  \
    \  }\n    fn from_raw(val: u32) -> Self {\n        Fp { val, _m: PhantomData }\n\
    \    }\n    pub fn value(self) -> u32 {\n        let v = reduce::<M>(self.val\
    \ as u64);\n        if v >= M::P { v - M::P } else { v }\n    }\n    pub fn grow(self)\
    \ -> FpGrow<M> {\n        FpGrow::from_raw((self.val as u64) << 32)\n    }\n \
    \   pub fn mul_unreduced<T: Into<Self>>(self, rhs: T) -> FpGrow<M> {\n       \
    \ FpGrow::from_raw(self.val as u64 * rhs.into().val as u64)\n    }\n    pub fn\
    \ pow<I: Int>(self, k: I) -> Self {\n        if self.val == 0 && k.is_zero() {\n\
    \            return Self::new(1);\n        }\n        let (mut e, mut k) = (self,\
    \ k.rem_euclid((M::P - 1).as_()));\n        let mut res = Self::ONE;\n       \
    \ while !k.is_zero() {\n            if !(k & 1.as_()).is_zero() {\n          \
    \      res *= e;\n            }\n            e *= e;\n            k >>= 1;\n \
    \       }\n        res\n    }\n    pub fn inv(self) -> Self {\n        let (mut\
    \ a, mut b, mut u, mut v) = (M::P as i32, self.value() as i32, 0, 1);\n      \
    \  while b != 0 {\n            let t = a / b;\n            a -= t * b;\n     \
    \       u -= t * v;\n            std::mem::swap(&mut a, &mut b);\n           \
    \ std::mem::swap(&mut u, &mut v);\n        }\n        debug_assert_eq!(a, 1);\n\
    \        if u < 0 {\n            debug_assert_eq!(v, M::P as i32);\n         \
    \   u += v;\n        }\n        Self::new(u as u32)\n    }\n}\n\n#[derive(Default,\
    \ Clone, Copy, PartialEq, Eq)]\npub struct FpGrow<M: Mod> {\n    val: u64,\n \
    \   _m: PhantomData<M>,\n}\n\nimpl<M: Mod> FpGrow<M> {\n    const MOD: u64 = (M::P\
    \ as u64) << 32;\n    fn from_raw(val: u64) -> Self {\n        Self { val, _m:\
    \ PhantomData }\n    }\n    pub fn reduce(self) -> Fp<M> {\n        Fp::from_raw(reduce::<M>(self.val))\n\
    \    }\n    pub fn value(self) -> u32 {\n        self.reduce().value()\n    }\n\
    }\n\nimpl<M: Mod> ops::Add<Self> for FpGrow<M> {\n    type Output = Self;\n  \
    \  fn add(mut self, rhs: Self) -> Self {\n        self += rhs;\n        self\n\
    \    }\n}\n\nimpl<M: Mod> ops::AddAssign<Self> for FpGrow<M> {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        self.val += rhs.val;\n        if self.val >= Self::MOD\
    \ * 2 {\n            self.val -= Self::MOD * 2;\n        }\n    }\n}\n\nimpl<M:\
    \ Mod> From<FpGrow<M>> for Fp<M> {\n    fn from(v: FpGrow<M>) -> Self {\n    \
    \    v.reduce()\n    }\n}\n\nimpl<M: Mod, I: Int> From<I> for Fp<M> {\n    fn\
    \ from(x: I) -> Self {\n        Self::new(x.rem_euclid(M::P.as_()).as_())\n  \
    \  }\n}\n\nimpl<M: Mod> cmp::PartialEq for Fp<M> {\n    fn eq(&self, other: &Self)\
    \ -> bool {\n        let val = |obj: &Fp<M>| {\n            if obj.val >= M::P\
    \ {\n                obj.val - M::P\n            } else {\n                obj.val\n\
    \            }\n        };\n        val(self) == val(other)\n    }\n}\n\nimpl<M:\
    \ Mod> cmp::Eq for Fp<M> {}\n\nimpl<M: Mod, T: Into<Fp<M>>> ops::AddAssign<T>\
    \ for Fp<M> {\n    fn add_assign(&mut self, rhs: T) {\n        self.val += rhs.into().val;\n\
    \        if self.val >= M::P * 2 {\n            self.val -= M::P * 2;\n      \
    \  }\n    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::SubAssign<T> for Fp<M> {\n \
    \   fn sub_assign(&mut self, rhs: T) {\n        let rhs = rhs.into();\n      \
    \  if self.val < rhs.val {\n            self.val += M::P * 2;\n        }\n   \
    \     self.val -= rhs.val;\n    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::MulAssign<T>\
    \ for Fp<M> {\n    fn mul_assign(&mut self, rhs: T) {\n        *self = self.mul_unreduced(rhs).reduce();\n\
    \    }\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T> for Fp<M> {\n    fn\
    \ div_assign(&mut self, rhs: T) {\n        *self *= rhs.into().inv();\n    }\n\
    }\n\nmacro_rules! impl_binop {\n    ($(($Op:ident, $op:ident, $OpAssign:ident,\
    \ $op_assign:ident)),*) => { $(\n        impl<M: Mod, T: Into<Fp<M>>> ops::$Op<T>\
    \ for Fp<M> {\n            type Output = Self;\n            fn $op(mut self, rhs:\
    \ T) -> Self {\n                ops::$OpAssign::$op_assign(&mut self, rhs); self\n\
    \            }\n        }\n    )* };\n}\n\nimpl_binop!(\n    (Add, add, AddAssign,\
    \ add_assign),\n    (Sub, sub, SubAssign, sub_assign),\n    (Mul, mul, MulAssign,\
    \ mul_assign),\n    (Div, div, DivAssign, div_assign)\n);\n\nimpl<M: Mod> ops::Neg\
    \ for Fp<M> {\n    type Output = Self;\n    fn neg(self) -> Self {\n        Fp::from_raw(M::P\
    \ * 2 - self.val)\n    }\n}\n\nimpl<M: Mod> iter::Sum for Fp<M> {\n    fn sum<I:\
    \ Iterator<Item = Self>>(iter: I) -> Self {\n        iter.fold(Self::ZERO, |b,\
    \ x| b + x)\n    }\n}\n\nimpl<M: Mod> iter::Product for Fp<M> {\n    fn product<I:\
    \ Iterator<Item = Self>>(iter: I) -> Self {\n        iter.fold(Self::ONE, |b,\
    \ x| b * x)\n    }\n}\n\nimpl<M: Mod> fmt::Debug for Fp<M> {\n    fn fmt(&self,\
    \ f: &mut fmt::Formatter) -> fmt::Result {\n        self.value().fmt(f)\n    }\n\
    }\n\nimpl<M: Mod> fmt::Display for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result {\n        self.value().fmt(f)\n    }\n}\n\nimpl<M: Mod> ZeroOne\
    \ for Fp<M> {\n    const ZERO: Self = Self { val: 0, _m: PhantomData };\n    const\
    \ ONE: Self = Self {\n        val: M::P.wrapping_neg() % M::P,\n        _m: PhantomData,\n\
    \    };\n}\n\nimpl<M: Mod> Num for Fp<M> {\n    fn wrapping_neg(self) -> Self\
    \ {\n        -self\n    }\n}\n\nimpl<M: Mod> Print for Fp<M> {\n    fn print(w:\
    \ &mut IO, x: Self) {\n        w.print(x.value());\n    }\n}\n\nimpl<M: Mod> Scan\
    \ for Fp<M> {\n    fn scan(io: &mut IO) -> Self {\n        Self::new(io.scan())\n\
    \    }\n}\n"
  dependsOn:
  - src/bits.rs
  - src/bounded.rs
  - src/cast.rs
  - src/int.rs
  - src/io.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/draft/fpacc64.rs
  requiredBy: []
  timestamp: '2021-02-15 17:55:41+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/fpacc64.rs
layout: document
redirect_from:
- /library/src/draft/fpacc64.rs
- /library/src/draft/fpacc64.rs.html
title: src/draft/fpacc64.rs
---
