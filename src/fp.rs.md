---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/fp/conv.rs
    title: src/fp/conv.rs
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
  code: "use crate::io::*;\npub use crate::zo::ZeroOne;\nuse std::marker::PhantomData;\n\
    use std::{cmp, fmt, iter, ops, u64, usize};\n\npub mod conv;\npub mod num;\n\n\
    pub trait Mod: Default + Clone + Copy + PartialEq + Eq {\n\tconst P: u32;\n\t\
    const K: u32; // -1 / P mod 2^32\n\tconst R2: u32; // 2^64 mod P\n}\n\n// montgomery\
    \ reduction (x -> x / 2^32 mod P)\nfn reduce<M: Mod>(x: u64) -> u32 {\n\tlet s\
    \ = M::K.wrapping_mul(x as u32);\n\t((x + s as u64 * M::P as u64) >> 32) as u32\n\
    }\n\n#[macro_export]\nmacro_rules! def_prime {\n\t($name:ident, $modu:expr, $k:expr)\
    \ => {\n\t\t#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n\t\tpub struct\
    \ $name;\n\t\timpl Mod for $name {\n\t\t\tconst P: u32 = $modu;\n\t\t\tconst K:\
    \ u32 = $k;\n\t\t\tconst R2: u32 = ((1_u128 << 64) % $modu) as u32;\n\t\t}\n\t\
    };\n}\n\ndef_prime!(ModA, 1_000_000_007, 2_226_617_417);\ndef_prime!(ModB, 998_244_353,\
    \ 998_244_351);\ndef_prime!(ModC, 1_012_924_417, 1_012_924_415);\ndef_prime!(ModD,\
    \ 924_844_033, 924_844_031);\n\n// modular arithmetics\n#[repr(transparent)]\n\
    #[derive(Default, Clone, Copy)]\npub struct Fp<M: Mod> {\n\tval: u32,\n\t_m: PhantomData<M>,\n\
    }\n\npub type FpA = Fp<ModA>;\npub type FpB = Fp<ModB>;\npub type FpC = Fp<ModC>;\n\
    pub type FpD = Fp<ModD>;\n\n/// mod 1_000_000_007\npub type F17 = FpA;\n\n///\
    \ mod 998_244_353\npub type F99 = FpB;\n\nimpl<M: Mod> Fp<M> {\n\tpub const P:\
    \ u32 = M::P;\n\tpub fn new(val: u32) -> Self {\n\t\tval.into()\n\t}\n\tfn from_raw(val:\
    \ u32) -> Self {\n\t\tFp { val, _m: PhantomData }\n\t}\n\tpub fn value(self) ->\
    \ u32 {\n\t\tlet v = reduce::<M>(self.val as u64);\n\t\tif v >= M::P { v - M::P\
    \ } else { v }\n\t}\n\tpub fn pow(mut self, mut k: u64) -> Self {\n\t\tif self.val\
    \ == 0 && k == 0 {\n\t\t\treturn Self::new(1);\n\t\t}\n\t\tk %= (M::P - 1) as\
    \ u64;\n\t\tlet mut res = Self::ONE;\n\t\twhile !k.is_zero() {\n\t\t\tif k % 2\
    \ != 0 {\n\t\t\t\tres *= self;\n\t\t\t}\n\t\t\tself *= self;\n\t\t\tk >>= 1;\n\
    \t\t}\n\t\tres\n\t}\n\tpub fn inv(self) -> Self {\n\t\tlet (mut a, mut b, mut\
    \ u, mut v) = (M::P as i32, self.value() as i32, 0, 1);\n\t\twhile b != 0 {\n\t\
    \t\tlet t = a / b;\n\t\t\ta -= t * b;\n\t\t\tu -= t * v;\n\t\t\tstd::mem::swap(&mut\
    \ a, &mut b);\n\t\t\tstd::mem::swap(&mut u, &mut v);\n\t\t}\n\t\tdebug_assert_eq!(a,\
    \ 1);\n\t\tif u < 0 {\n\t\t\tdebug_assert_eq!(v, M::P as i32);\n\t\t\tu += v;\n\
    \t\t}\n\t\tSelf::new(u as u32)\n\t}\n}\n\nimpl<M: Mod> From<u32> for Fp<M> {\n\
    \tfn from(x: u32) -> Self {\n\t\tFp::from_raw(reduce::<M>(x as u64 * M::R2 as\
    \ u64))\n\t}\n}\n\nimpl<M: Mod> From<usize> for Fp<M> {\n\tfn from(x: usize) ->\
    \ Self {\n\t\tFp::from_raw(reduce::<M>(x as u64 * M::R2 as u64))\n\t}\n}\n\nmacro_rules!\
    \ impl_from_int {\n\t($($t:ty),*) => { $(\n\t\timpl<M: Mod> From<$t> for Fp<M>\
    \ {\n\t\t\tfn from(x: $t) -> Self {\n\t\t\t\tFp::from_raw(reduce::<M>(x.rem_euclid(M::P\
    \ as _) as u64 * M::R2 as u64))\n\t\t\t}\n\t\t}\n\t)* };\n}\n\nimpl_from_int!(u64,\
    \ i32, i64, isize);\n\nimpl<M: Mod> cmp::PartialEq for Fp<M> {\n\tfn eq(&self,\
    \ other: &Self) -> bool {\n\t\tlet val = |obj: &Fp<M>| {\n\t\t\tif obj.val >=\
    \ M::P {\n\t\t\t\tobj.val - M::P\n\t\t\t} else {\n\t\t\t\tobj.val\n\t\t\t}\n\t\
    \t};\n\t\tval(self) == val(other)\n\t}\n}\n\nimpl<M: Mod> cmp::Eq for Fp<M> {}\n\
    \nimpl<M: Mod, T: Into<Fp<M>>> ops::AddAssign<T> for Fp<M> {\n\tfn add_assign(&mut\
    \ self, rhs: T) {\n\t\tself.val += rhs.into().val;\n\t\tif self.val >= M::P *\
    \ 2 {\n\t\t\tself.val -= M::P * 2;\n\t\t}\n\t}\n}\nimpl<M: Mod, T: Into<Fp<M>>>\
    \ ops::SubAssign<T> for Fp<M> {\n\tfn sub_assign(&mut self, rhs: T) {\n\t\tlet\
    \ rhs = rhs.into();\n\t\tif self.val < rhs.val {\n\t\t\tself.val += M::P * 2;\n\
    \t\t}\n\t\tself.val -= rhs.val;\n\t}\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::MulAssign<T>\
    \ for Fp<M> {\n\tfn mul_assign(&mut self, rhs: T) {\n\t\tself.val = reduce::<M>(self.val\
    \ as u64 * rhs.into().val as u64);\n\t}\n}\nimpl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T>\
    \ for Fp<M> {\n\tfn div_assign(&mut self, rhs: T) {\n\t\t*self *= rhs.into().inv();\n\
    \t}\n}\n\nmacro_rules! impl_binop {\n\t($(($Op:ident, $op:ident, $OpAssign:ident,\
    \ $op_assign:ident)),*) => { $(\n\t\timpl<M: Mod, T: Into<Fp<M>>> ops::$Op<T>\
    \ for Fp<M> {\n\t\t\ttype Output = Self;\n\t\t\tfn $op(mut self, rhs: T) -> Self\
    \ {\n\t\t\t\tops::$OpAssign::$op_assign(&mut self, rhs); self\n\t\t\t}\n\t\t}\n\
    \t)* };\n}\n\nimpl_binop!(\n\t(Add, add, AddAssign, add_assign),\n\t(Sub, sub,\
    \ SubAssign, sub_assign),\n\t(Mul, mul, MulAssign, mul_assign),\n\t(Div, div,\
    \ DivAssign, div_assign)\n);\n\nimpl<M: Mod> ops::Neg for Fp<M> {\n\ttype Output\
    \ = Self;\n\tfn neg(self) -> Self {\n\t\tFp::from_raw(M::P * 2 - self.val)\n\t\
    }\n}\n\nimpl<M: Mod> iter::Sum for Fp<M> {\n\tfn sum<I: Iterator<Item = Self>>(iter:\
    \ I) -> Self {\n\t\titer.fold(Self::ZERO, |b, x| b + x)\n\t}\n}\n\nimpl<M: Mod>\
    \ iter::Product for Fp<M> {\n\tfn product<I: Iterator<Item = Self>>(iter: I) ->\
    \ Self {\n\t\titer.fold(Self::ONE, |b, x| b * x)\n\t}\n}\n\nimpl<M: Mod> fmt::Debug\
    \ for Fp<M> {\n\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n\t\t\
    self.value().fmt(f)\n\t}\n}\n\nimpl<M: Mod> fmt::Display for Fp<M> {\n\tfn fmt(&self,\
    \ f: &mut fmt::Formatter) -> fmt::Result {\n\t\tself.value().fmt(f)\n\t}\n}\n\n\
    impl<M: Mod> ZeroOne for Fp<M> {\n\tconst ZERO: Self = Self { val: 0, _m: PhantomData\
    \ };\n\tconst ONE: Self = Self { val: M::P.wrapping_neg() % M::P, _m: PhantomData\
    \ };\n}\n\nimpl<M: Mod> Print for Fp<M> {\n\tfn print(w: &mut IO, x: Self) {\n\
    \t\tw.print(x.value());\n\t}\n}\n\nimpl<M: Mod> Scan for Fp<M> {\n\tfn scan(io:\
    \ &mut IO) -> Self {\n\t\tSelf::new(io.scan())\n\t}\n}\n"
  dependsOn:
  - src/io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/fp.rs
  requiredBy:
  - src/fp/conv.rs
  - src/fp/num.rs
  - src/u64/conv.rs
  - src/tests.rs
  timestamp: '2021-01-27 22:59:31+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_garner_test.rs
documentation_of: src/fp.rs
layout: document
redirect_from:
- /library/src/fp.rs
- /library/src/fp.rs.html
title: src/fp.rs
---