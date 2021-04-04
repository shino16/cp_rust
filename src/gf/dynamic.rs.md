---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/math/gcd/ext.rs
    title: src/math/gcd/ext.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/gf/dynamic.rs\n"
  code: "// modular arithmetics\n\nuse crate::zo::ZeroOne;\nuse crate::math::gcd::ext::extgcd;\n\
    use std::marker::PhantomData;\nuse std::{cmp, fmt, iter, ops::*, str, u32};\n\n\
    pub trait DMod: Default + Clone + Copy {\n    fn p() -> u32;  // odd (not necessarily\
    \ prime)\n    fn k() -> u32;  // -1 / P mod 2^32\n    fn r2() -> u32; // 2^64\
    \ mod P\n    unsafe fn set_mod(p: u32);  // may cause data race\n}\n\n// montgomery\
    \ reduction (x -> x / 2^32 mod P)\nfn reduce<M: DMod>(x: u64) -> u32 {\n    let\
    \ s = M::k().wrapping_mul(x as u32);\n    ((x + s as u64 * M::p() as u64) >> 32)\
    \ as u32\n}\n\n#[macro_export]\nmacro_rules! def_dyn_mod {\n    ($Name:ident,\
    \ $name:ident) => {\n        mod $name {\n            use super::*;\n        \
    \    use std::cell::Cell;\n\n            #[derive(Default, Clone, Copy)]\n   \
    \         pub struct $Name;\n\n            struct SyncCell<T>(Cell<T>);\n    \
    \        unsafe impl<T> Sync for SyncCell<T> {}  // not safe\n\n            static\
    \ P: SyncCell<u32> = SyncCell(Cell::new(1));\n            static K: SyncCell<u32>\
    \ = SyncCell(Cell::new(!0));\n            static R2: SyncCell<u32> = SyncCell(Cell::new(0));\n\
    \n            impl DMod for $Name {\n                fn p() -> u32 { P.0.get()\
    \ }\n                fn k() -> u32 { K.0.get() }\n                fn r2() -> u32\
    \ { R2.0.get() }\n                unsafe fn set_mod(p: u32) {\n              \
    \      #[cfg(debug_assertions)]\n                    eprintln!(\"value of {} is\
    \ set to {}\", std::stringify!($Name), p);\n                    P.0.set(p);\n\
    \                    let (g, nk) = extgcd(p as u64, 1 << 32);\n              \
    \      assert_eq!(g, 1);\n                    K.0.set((nk as u32).wrapping_neg());\n\
    \                    R2.0.set(((p as u64).wrapping_neg() % p as u64) as u32);\n\
    \                }\n            }\n        }\n        pub use self::$name::$Name;\n\
    \    };\n}\n\ndef_dyn_mod!(DefaultMod, default_mod);\n\n#[repr(transparent)]\n\
    #[derive(Default, Clone, Copy)]\npub struct DynGf<M: DMod = DefaultMod> {\n  \
    \  val: u32,\n    _m: PhantomData<M>,\n}\n\nimpl<M: DMod> DynGf<M> {\n    pub\
    \ unsafe fn set_mod(p: u32) { M::set_mod(p); }\n    pub fn new(val: u32) -> Self\
    \ { val.into() }\n    pub fn zero() -> Self { Self::from_raw(0) }\n    pub fn\
    \ one() -> Self { 1.into() }\n    fn from_raw(val: u32) -> Self { DynGf { val,\
    \ _m: PhantomData } }\n    pub fn value(self) -> u32 {\n        let v = reduce::<M>(self.val\
    \ as u64);\n        if v >= M::p() { v - M::p() } else { v }\n    }\n    pub fn\
    \ pow(mut self, mut k: u64) -> Self {\n        if self.val == 0 && k == 0 { return\
    \ Self::new(1); }\n        k %= (M::p() - 1) as u64;\n        let mut res = Self::one();\n\
    \        while !k.is_zero() {\n            if k % 2 != 0 { res *= self; }\n  \
    \          self *= self; k >>= 1;\n        }\n        res\n    }\n    pub fn inv(self)\
    \ -> Self {\n        let (mut a, mut b, mut u, mut v) = (M::p() as i32, self.value()\
    \ as i32, 0, 1);\n        while b != 0 {\n            let t = a / b;\n       \
    \     a -= t * b; u -= t * v;\n            std::mem::swap(&mut a, &mut b); std::mem::swap(&mut\
    \ u, &mut v);\n        }\n        debug_assert_eq!(a, 1);\n        if u < 0 {\
    \ debug_assert_eq!(v, M::p() as i32); u += v; }\n        Self::new(u as u32)\n\
    \    }\n}\nimpl<M: DMod> From<u32> for DynGf<M> {\n    fn from(x: u32) -> Self\
    \ { DynGf::from_raw(reduce::<M>(x as u64 * M::r2() as u64)) }\n}\nmacro_rules!\
    \ impl_from_int {\n    ($($t:ty),*) => { $(\n        impl<M: DMod> From<$t> for\
    \ DynGf<M> {\n            fn from(x: $t) -> Self {\n                DynGf::from_raw(reduce::<M>(x.rem_euclid(M::p()\
    \ as _) as u64 * M::r2() as u64))\n            }\n        }\n    )* };\n}\nimpl_from_int!(u64,\
    \ usize, i32, i64, isize);\nimpl<M: DMod> cmp::PartialEq for DynGf<M> {\n    fn\
    \ eq(&self, other: &Self) -> bool {\n        let val = |obj: &DynGf<M>| {\n  \
    \          if obj.val >= M::p() { obj.val - M::p() } else { obj.val }\n      \
    \  };\n        val(self) == val(other)\n    }\n}\nimpl<M: DMod> cmp::Eq for DynGf<M>\
    \ {}\nimpl<M: DMod, T: Into<DynGf<M>>> AddAssign<T> for DynGf<M> {\n    fn add_assign(&mut\
    \ self, rhs: T) {\n        self.val += rhs.into().val;\n        if self.val >=\
    \ M::p() * 2 { self.val -= M::p() * 2; }\n    }\n}\nimpl<M: DMod, T: Into<DynGf<M>>>\
    \ SubAssign<T> for DynGf<M> {\n    fn sub_assign(&mut self, rhs: T) {\n      \
    \  let rhs = rhs.into();\n        if self.val < rhs.val { self.val += M::p() *\
    \ 2; }\n        self.val -= rhs.val;\n    }\n}\nimpl<M: DMod, T: Into<DynGf<M>>>\
    \ MulAssign<T> for DynGf<M> {\n    fn mul_assign(&mut self, rhs: T) {\n      \
    \  self.val = reduce::<M>(self.val as u64 * rhs.into().val as u64);\n    }\n}\n\
    impl<M: DMod, T: Into<DynGf<M>>> DivAssign<T> for DynGf<M> {\n    fn div_assign(&mut\
    \ self, rhs: T) { *self *= rhs.into().inv(); }\n}\nmacro_rules! impl_binop {\n\
    \    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(\n\
    \        impl<M: DMod, T: Into<DynGf<M>>> $Op<T> for DynGf<M> {\n            type\
    \ Output = Self;\n            fn $op(mut self, rhs: T) -> Self { self.$op_assign(rhs);\
    \ self }\n        }\n    )* };\n}\nimpl_binop!(\n    (Add, add, AddAssign, add_assign),\n\
    \    (Sub, sub, SubAssign, sub_assign),\n    (Mul, mul, MulAssign, mul_assign),\n\
    \    (Div, div, DivAssign, div_assign)\n);\nimpl<M: DMod> Neg for DynGf<M> {\n\
    \    type Output = Self;\n    fn neg(self) -> Self { DynGf::from_raw(M::p() *\
    \ 2 - self.val) }\n}\nimpl<M: DMod> iter::Sum for DynGf<M> {\n    fn sum<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self { iter.fold(Self::zero(), |b, x| b + x) }\n}\nimpl<M:\
    \ DMod> iter::Product for DynGf<M> {\n    fn product<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self { iter.fold(Self::one(), |b, x| b * x) }\n}\nimpl<M: DMod> fmt::Debug\
    \ for DynGf<M> {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f)\
    \ }\n}\nimpl<M: DMod> fmt::Display for DynGf<M> {\n    fn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result { self.value().fmt(f) }\n}\nimpl<M: DMod> str::FromStr for DynGf<M>\
    \ {\n    type Err = <u32 as str::FromStr>::Err;\n    fn from_str(s: &str) -> Result<Self,\
    \ Self::Err> { u32::from_str(s).map(Self::new) }\n}\n"
  dependsOn:
  - src/math/gcd/ext.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/gf/dynamic.rs
  requiredBy: []
  timestamp: '2021-04-04 11:43:54+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/gf/dynamic.rs
layout: document
redirect_from:
- /library/src/gf/dynamic.rs
- /library/src/gf/dynamic.rs.html
title: src/gf/dynamic.rs
---
