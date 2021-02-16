---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
  - icon: ':warning:'
    path: src/fp/io.rs
    title: src/fp/io.rs
  - icon: ':warning:'
    path: src/fp/num.rs
    title: src/fp/num.rs
  - icon: ':heavy_check_mark:'
    path: src/tests.rs
    title: src/tests.rs
  - icon: ':warning:'
    path: src/u64/conv.rs
    title: src/u64/conv.rs
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
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/fp.rs\n"
  code: "// modular arithmetics\n\npub use crate::zo::ZeroOne;\nuse std::marker::PhantomData;\n\
    use std::{cmp, fmt, iter, ops};\n\npub mod conv;\npub mod io;\npub mod num;\n\n\
    pub trait Mod: Default + Clone + Copy + PartialEq + Eq {\n    const P: u32;\n\
    \    const K: u32; // -1 / P mod 2^32\n    const R2: u32; // 2^64 mod P\n}\n\n\
    // montgomery reduction (x -> x / 2^32 mod P)\nfn reduce<M: Mod>(x: u64) -> u32\
    \ {\n    let s = M::K.wrapping_mul(x as u32);\n    ((x + s as u64 * M::P as u64)\
    \ >> 32) as u32\n}\n\n#[macro_export]\nmacro_rules! def_prime {\n    ($name:ident,\
    \ $modu:expr, $k:expr) => {\n        #[derive(Default, Clone, Copy, PartialEq,\
    \ Eq, Debug)]\n        pub struct $name;\n        impl Mod for $name {\n     \
    \       const P: u32 = $modu;\n            const K: u32 = $k;\n            const\
    \ R2: u32 = ((1_u128 << 64) % $modu) as u32;\n        }\n    };\n}\n\ndef_prime!(ModA,\
    \ 1_000_000_007, 2_226_617_417);\ndef_prime!(ModB, 998_244_353, 998_244_351);\n\
    def_prime!(ModC, 1_012_924_417, 1_012_924_415);\ndef_prime!(ModD, 924_844_033,\
    \ 924_844_031);\n\n#[repr(transparent)]\n#[derive(Default, Clone, Copy)]\npub\
    \ struct Fp<M: Mod> {\n    val: u32,\n    _m: PhantomData<M>,\n}\n\npub type FpA\
    \ = Fp<ModA>;\npub type FpB = Fp<ModB>;\npub type FpC = Fp<ModC>;\npub type FpD\
    \ = Fp<ModD>;\npub type F17 = FpA;\npub type F99 = FpB;\n\nimpl<M: Mod> Fp<M>\
    \ {\n    pub const P: u32 = M::P;\n    pub fn new(val: u32) -> Self { val.into()\
    \ }\n    fn from_raw(val: u32) -> Self { Fp { val, _m: PhantomData } }\n    pub\
    \ fn value(self) -> u32 {\n        let v = reduce::<M>(self.val as u64);\n   \
    \     if v >= M::P { v - M::P } else { v }\n    }\n    pub fn pow(mut self, mut\
    \ k: u64) -> Self {\n        if self.val == 0 && k == 0 {\n            return\
    \ Self::new(1);\n        }\n        k %= (M::P - 1) as u64;\n        let mut res\
    \ = Self::ONE;\n        while !k.is_zero() {\n            if k % 2 != 0 { res\
    \ *= self; }\n            self *= self;\n            k >>= 1;\n        }\n   \
    \     res\n    }\n    pub fn inv(self) -> Self {\n        let (mut a, mut b, mut\
    \ u, mut v) = (M::P as i32, self.value() as i32, 0, 1);\n        while b != 0\
    \ {\n            let t = a / b;\n            a -= t * b;\n            u -= t *\
    \ v;\n            std::mem::swap(&mut a, &mut b);\n            std::mem::swap(&mut\
    \ u, &mut v);\n        }\n        debug_assert_eq!(a, 1);\n        if u < 0 {\n\
    \            debug_assert_eq!(v, M::P as i32);\n            u += v;\n        }\n\
    \        Self::new(u as u32)\n    }\n}\nimpl<M: Mod> From<u32> for Fp<M> {\n \
    \   fn from(x: u32) -> Self { Fp::from_raw(reduce::<M>(x as u64 * M::R2 as u64))\
    \ }\n}\nmacro_rules! impl_from_int {\n    ($($t:ty),*) => { $(\n        impl<M:\
    \ Mod> From<$t> for Fp<M> {\n            fn from(x: $t) -> Self {\n          \
    \      Fp::from_raw(reduce::<M>(x.rem_euclid(M::P as _) as u64 * M::R2 as u64))\n\
    \            }\n        }\n    )* };\n}\nimpl_from_int!(u64, i32, i64);\nimpl<M:\
    \ Mod> cmp::PartialEq for Fp<M> {\n    fn eq(&self, other: &Self) -> bool {\n\
    \        let val = |obj: &Fp<M>| {\n            if obj.val >= M::P { obj.val -\
    \ M::P } else { obj.val }\n        };\n        val(self) == val(other)\n    }\n\
    }\nimpl<M: Mod> cmp::Eq for Fp<M> {}\nimpl<M: Mod, T: Into<Fp<M>>> ops::AddAssign<T>\
    \ for Fp<M> {\n    fn add_assign(&mut self, rhs: T) {\n        self.val += rhs.into().val;\n\
    \        if self.val >= M::P * 2 { self.val -= M::P * 2; }\n    }\n}\nimpl<M:\
    \ Mod, T: Into<Fp<M>>> ops::SubAssign<T> for Fp<M> {\n    fn sub_assign(&mut self,\
    \ rhs: T) {\n        let rhs = rhs.into();\n        if self.val < rhs.val { self.val\
    \ += M::P * 2; }\n        self.val -= rhs.val;\n    }\n}\nimpl<M: Mod, T: Into<Fp<M>>>\
    \ ops::MulAssign<T> for Fp<M> {\n    fn mul_assign(&mut self, rhs: T) {\n    \
    \    self.val = reduce::<M>(self.val as u64 * rhs.into().val as u64);\n    }\n\
    }\nimpl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T> for Fp<M> {\n    fn div_assign(&mut\
    \ self, rhs: T) { *self *= rhs.into().inv(); }\n}\nmacro_rules! impl_binop {\n\
    \    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(\n\
    \        impl<M: Mod, T: Into<Fp<M>>> ops::$Op<T> for Fp<M> {\n            type\
    \ Output = Self;\n            fn $op(mut self, rhs: T) -> Self { ops::$OpAssign::$op_assign(&mut\
    \ self, rhs); self }\n        }\n    )* };\n}\nimpl_binop!(\n    (Add, add, AddAssign,\
    \ add_assign),\n    (Sub, sub, SubAssign, sub_assign),\n    (Mul, mul, MulAssign,\
    \ mul_assign),\n    (Div, div, DivAssign, div_assign)\n);\nimpl<M: Mod> ops::Neg\
    \ for Fp<M> {\n    type Output = Self;\n    fn neg(self) -> Self { Fp::from_raw(M::P\
    \ * 2 - self.val) }\n}\nimpl<M: Mod> iter::Sum for Fp<M> {\n    fn sum<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self { iter.fold(Self::ZERO, |b, x| b + x) }\n}\nimpl<M:\
    \ Mod> iter::Product for Fp<M> {\n    fn product<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self { iter.fold(Self::ONE, |b, x| b * x) }\n}\nimpl<M: Mod> fmt::Debug\
    \ for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f)\
    \ }\n}\nimpl<M: Mod> fmt::Display for Fp<M> {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result { self.value().fmt(f) }\n}\nimpl<M: Mod> ZeroOne for Fp<M> {\n\
    \    const ZERO: Self = Self { val: 0, _m: PhantomData };\n    const ONE: Self\
    \ = Self { val: M::P.wrapping_neg() % M::P, _m: PhantomData };\n}\n"
  dependsOn:
  - src/zo.rs
  isVerificationFile: false
  path: src/fp.rs
  requiredBy:
  - src/u64/conv.rs
  - src/fp/conv.rs
  - src/fp/io.rs
  - src/fp/num.rs
  - src/tests.rs
  timestamp: '2021-02-15 17:55:41+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_garner_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_test.rs
documentation_of: src/fp.rs
layout: document
redirect_from:
- /library/src/fp.rs
- /library/src/fp.rs.html
title: src/fp.rs
---
