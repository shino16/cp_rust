---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/mint/conv.rs
    title: src/mint/conv.rs
  - icon: ':warning:'
    path: src/mint/num.rs
    title: src/mint/num.rs
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
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
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/mint.rs\n"
  code: "use crate::io::*;\npub use crate::zo::ZeroOne;\nuse std::marker::PhantomData;\n\
    use std::{fmt, iter, ops};\n\npub mod conv;\npub mod num;\n\npub trait Mod: Default\
    \ + Clone + Copy + PartialEq + Eq {\n\tconst M: u32;\n\tconst PHI: u32;\n}\n\n\
    #[macro_export]\nmacro_rules! def_mod {\n\t($name:ident, $modulus:expr) => {\n\
    \t\tdef_mod!($name, $modulus, $modulus - 1);\n\t};\n\t($name:ident, $modulus:expr,\
    \ $phi:expr) => {\n\t\t#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n\
    \t\tpub struct $name;\n\t\timpl Mod for $name {\n\t\t\tconst M: u32 = $modulus;\n\
    \t\t\tconst PHI: u32 = $phi;\n\t\t}\n\t};\n}\n\ndef_mod!(ModA, 1_000_000_007);\n\
    def_mod!(ModB, 998_244_353);\ndef_mod!(ModC, 1_012_924_417);\ndef_mod!(ModD, 924_844_033);\n\
    \n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct Mint<M: Mod> {\n\t\
    pub val: u32,\n\t_m: PhantomData<M>,\n}\n\npub type MintA = Mint<ModA>;\npub type\
    \ MintB = Mint<ModB>;\npub type MintC = Mint<ModC>;\npub type MintD = Mint<ModD>;\n\
    \npub type Mint17 = MintA;\npub type Mint99 = MintB;\n\nimpl<M: Mod> Mint<M> {\n\
    \tpub const M: u32 = M::M;\n\tpub fn new(val: i64) -> Self {\n\t\tSelf::from_val(val.rem_euclid(M::M\
    \ as i64) as u32)\n\t}\n\tpub fn from_val(val: u32) -> Self {\n\t\tMint { val,\
    \ _m: PhantomData }\n\t}\n\tpub fn value(self) -> u32 {\n\t\tself.val\n\t}\n\t\
    pub fn pow(self, mut exp: u32) -> Self {\n\t\tif self.val == 0 && exp == 0 {\n\
    \t\t\treturn Self::from_val(1);\n\t\t}\n\t\tlet mut e = self;\n\t\tlet mut res\
    \ = Self::from_val(1);\n\t\twhile exp != 0 {\n\t\t\tif exp % 2 == 1 {\n\t\t\t\t\
    res *= e;\n\t\t\t}\n\t\t\te *= e;\n\t\t\texp >>= 1;\n\t\t}\n\t\tres\n\t}\n\tpub\
    \ fn inv(self) -> Self {\n\t\tself.pow(M::PHI - 1)\n\t}\n\tpub fn modulus() ->\
    \ u32 {\n\t\tM::M\n\t}\n}\n\nmacro_rules! impl_from_int {\n\t($(($t:ty: $via:ty)),*)\
    \ => { $(\n\t\timpl<M: Mod> From<$t> for Mint<M> {\n\t\t\tfn from(x: $t) -> Self\
    \ {\n\t\t\t\tSelf::from_val((x as $via).rem_euclid(M::M as $via) as u32)\n\t\t\
    \t}\n\t\t}\n\t)* };\n}\n\nimpl_from_int! {\n\t(i8: i32), (i16: i32), (i32: i32),\
    \ (i64: i64), (isize: isize),\n\t(u8: u32), (u16: u32), (u32: u32), (u64: u64),\
    \ (usize: usize)\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::Add<T> for Mint<M>\
    \ {\n\ttype Output = Self;\n\tfn add(mut self, rhs: T) -> Self {\n\t\tself +=\
    \ rhs;\n\t\tself\n\t}\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::AddAssign<T> for\
    \ Mint<M> {\n\tfn add_assign(&mut self, rhs: T) {\n\t\tself.val += rhs.into().val;\n\
    \t\tif self.val >= M::M {\n\t\t\tself.val -= M::M;\n\t\t}\n\t}\n}\n\nimpl<M: Mod>\
    \ ops::Neg for Mint<M> {\n\ttype Output = Self;\n\tfn neg(self) -> Self {\n\t\t\
    Mint::from_val(match self.val {\n\t\t\t0 => 0,\n\t\t\ts => M::M - s,\n\t\t})\n\
    \t}\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::Sub<T> for Mint<M> {\n\ttype Output\
    \ = Self;\n\tfn sub(mut self, rhs: T) -> Self {\n\t\tself -= rhs;\n\t\tself\n\t\
    }\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::SubAssign<T> for Mint<M> {\n\tfn sub_assign(&mut\
    \ self, rhs: T) {\n\t\tlet rhs = rhs.into();\n\t\tif self.val < rhs.val {\n\t\t\
    \tself.val += M::M;\n\t\t}\n\t\tself.val -= rhs.val;\n\t}\n}\n\nimpl<M: Mod, T:\
    \ Into<Mint<M>>> ops::Mul<T> for Mint<M> {\n\ttype Output = Self;\n\tfn mul(self,\
    \ rhs: T) -> Self {\n\t\tlet val = self.val as u64 * rhs.into().val as u64 % M::M\
    \ as u64;\n\t\tSelf::from_val(val as u32)\n\t}\n}\n\nimpl<M: Mod, T: Into<Mint<M>>>\
    \ ops::MulAssign<T> for Mint<M> {\n\tfn mul_assign(&mut self, rhs: T) {\n\t\t\
    *self = *self * rhs;\n\t}\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::Div<T> for\
    \ Mint<M> {\n\ttype Output = Self;\n\tfn div(mut self, rhs: T) -> Self {\n\t\t\
    self /= rhs;\n\t\tself\n\t}\n}\n\nimpl<M: Mod, T: Into<Mint<M>>> ops::DivAssign<T>\
    \ for Mint<M> {\n\tfn div_assign(&mut self, rhs: T) {\n\t\t*self *= rhs.into().pow(M::PHI\
    \ - 1);\n\t}\n}\n\nimpl<M: Mod> iter::Sum for Mint<M> {\n\tfn sum<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self {\n\t\titer.fold(Self::from_val(0), |b, x| b + x)\n\
    \t}\n}\n\nimpl<M: Mod> iter::Product for Mint<M> {\n\tfn product<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self {\n\t\titer.fold(Self::from_val(1), |b, x| b * x)\n\
    \t}\n}\n\nimpl<M: Mod> fmt::Debug for Mint<M> {\n\tfn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result {\n\t\tself.val.fmt(f)\n\t}\n}\n\nimpl<M: Mod> fmt::Display for\
    \ Mint<M> {\n\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n\t\tself.val.fmt(f)\n\
    \t}\n}\n\nimpl<M: Mod> ZeroOne for Mint<M> {\n\tconst ZERO: Self = Self { val:\
    \ 0, _m: PhantomData };\n\tconst ONE: Self = Self { val: 1, _m: PhantomData };\n\
    }\n\nimpl<M: Mod> Print for Mint<M> {\n\tfn print(w: &mut IO, x: Self) {\n\t\t\
    w.print(x.value());\n\t}\n}\n\nimpl<M: Mod> Scan for Mint<M> {\n\tfn scan(io:\
    \ &mut IO) -> Self {\n\t\tio.scan::<u32>().into()\n\t}\n}\n"
  dependsOn:
  - src/io.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/mint.rs
  requiredBy:
  - src/tests.rs
  - src/mint/conv.rs
  - src/mint/num.rs
  timestamp: '2021-01-27 17:46:37+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/lazy_segtree_test.rs
documentation_of: src/mint.rs
layout: document
redirect_from:
- /library/src/mint.rs
- /library/src/mint.rs.html
title: src/mint.rs
---
