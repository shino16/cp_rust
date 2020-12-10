---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_garner_test.rs
    title: test/src/bin/ntt_mint_garner_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_mint_test.rs
    title: test/src/bin/ntt_mint_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::int::ZeroOne;\nuse crate::int::*;\nuse crate::io::*;\nuse\
    \ std::marker::PhantomData;\nuse std::{fmt, iter, ops};\n\npub mod conv;\n\npub\
    \ trait Mod: Default + Clone + Copy + PartialEq + Eq {\n\tconst P: u32;\n\tconst\
    \ PHI: u32;\n}\n\nmacro_rules! def_mod {\n\t($name:ident, $modulus:expr) => {\n\
    \t\tdef_mod!($name, $modulus, $modulus - 1);\n\t};\n\t($name:ident, $modulus:expr,\
    \ $phi:expr) => {\n\t\t#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]\n\
    \t\tpub struct $name;\n\t\timpl Mod for $name {\n\t\t\tconst P: u32 = $modulus;\n\
    \t\t\tconst PHI: u32 = $phi;\n\t\t}\n\t};\n}\n\ndef_mod!(ModA, 1_000_000_007);\n\
    def_mod!(ModB, 998_244_353);\ndef_mod!(ModC, 1_012_924_417);\ndef_mod!(ModD, 924_844_033);\n\
    \n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct Mint<P: Mod> {\n\t\
    pub val: u32,\n\t_m: PhantomData<P>,\n}\n\npub type MintA = Mint<ModA>;\npub type\
    \ MintB = Mint<ModB>;\npub type MintC = Mint<ModC>;\npub type MintD = Mint<ModD>;\n\
    \npub type Mint17 = MintA;\npub type Mint99 = MintB;\n\nimpl<P: Mod> Mint<P> {\n\
    \tconst P: u32 = P::P;\n\tpub fn new(val: i64) -> Self {\n\t\tSelf::from_val(val.rem_euclid(P::P\
    \ as i64) as u32)\n\t}\n\tpub fn from_val(val: u32) -> Self {\n\t\tMint { val,\
    \ _m: PhantomData }\n\t}\n\tpub fn value(self) -> u32 {\n\t\tself.val\n\t}\n\t\
    pub fn pow(self, mut exp: u32) -> Self {\n\t\tif self.val == 0 && exp == 0 {\n\
    \t\t\treturn Self::from_val(1);\n\t\t}\n\t\tlet mut e = self;\n\t\tlet mut res\
    \ = Self::from_val(1);\n\t\twhile exp != 0 {\n\t\t\tif exp % 2 == 1 {\n\t\t\t\t\
    res *= e;\n\t\t\t}\n\t\t\te *= e;\n\t\t\texp >>= 1;\n\t\t}\n\t\tres\n\t}\n\tpub\
    \ fn inv(self) -> Self {\n\t\tself.pow(P::PHI - 1)\n\t}\n\tpub fn modulus() ->\
    \ u32 {\n\t\tP::P\n\t}\n}\n\nmacro_rules! impl_from_int {\n\t($(($t:ty: $via:ty)),*)\
    \ => { $(\n\t\timpl<P: Mod> From<$t> for Mint<P> {\n\t\t\tfn from(x: $t) -> Self\
    \ {\n\t\t\t\tSelf::from_val((x as $via).rem_euclid(P::P as $via) as u32)\n\t\t\
    \t}\n\t\t}\n\t)* };\n}\n\nimpl_from_int! {\n\t(i8: i32), (i16: i32), (i32: i32),\
    \ (i64: i64), (isize: isize),\n\t(u8: u32), (u16: u32), (u32: u32), (u64: u64),\
    \ (usize: usize)\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::Add<T> for Mint<P>\
    \ {\n\ttype Output = Self;\n\tfn add(mut self, rhs: T) -> Self {\n\t\tself +=\
    \ rhs;\n\t\tself\n\t}\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::AddAssign<T> for\
    \ Mint<P> {\n\tfn add_assign(&mut self, rhs: T) {\n\t\tself.val += rhs.into().val;\n\
    \t\tif self.val >= P::P {\n\t\t\tself.val -= P::P;\n\t\t}\n\t}\n}\n\nimpl<P: Mod>\
    \ ops::Neg for Mint<P> {\n\ttype Output = Self;\n\tfn neg(self) -> Self {\n\t\t\
    Mint::from_val(match self.val {\n\t\t\t0 => 0,\n\t\t\ts => P::P - s,\n\t\t})\n\
    \t}\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::Sub<T> for Mint<P> {\n\ttype Output\
    \ = Self;\n\tfn sub(mut self, rhs: T) -> Self {\n\t\tself -= rhs;\n\t\tself\n\t\
    }\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::SubAssign<T> for Mint<P> {\n\tfn sub_assign(&mut\
    \ self, rhs: T) {\n\t\tlet rhs = rhs.into();\n\t\tif self.val < rhs.val {\n\t\t\
    \tself.val += P::P;\n\t\t}\n\t\tself.val -= rhs.val;\n\t}\n}\n\nimpl<P: Mod, T:\
    \ Into<Mint<P>>> ops::Mul<T> for Mint<P> {\n\ttype Output = Self;\n\tfn mul(self,\
    \ rhs: T) -> Self {\n\t\tlet val = self.val as u64 * rhs.into().val as u64 % P::P\
    \ as u64;\n\t\tSelf::from_val(val as u32)\n\t}\n}\n\nimpl<P: Mod, T: Into<Mint<P>>>\
    \ ops::MulAssign<T> for Mint<P> {\n\tfn mul_assign(&mut self, rhs: T) {\n\t\t\
    *self = *self * rhs;\n\t}\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::Div<T> for\
    \ Mint<P> {\n\ttype Output = Self;\n\tfn div(mut self, rhs: T) -> Self {\n\t\t\
    self /= rhs;\n\t\tself\n\t}\n}\n\nimpl<P: Mod, T: Into<Mint<P>>> ops::DivAssign<T>\
    \ for Mint<P> {\n\tfn div_assign(&mut self, rhs: T) {\n\t\t*self *= rhs.into().pow(P::PHI\
    \ - 1);\n\t}\n}\n\nimpl<P: Mod> iter::Sum for Mint<P> {\n\tfn sum<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self {\n\t\titer.fold(Self::from_val(0), |b, x| b + x)\n\
    \t}\n}\n\nimpl<P: Mod> iter::Product for Mint<P> {\n\tfn product<I: Iterator<Item\
    \ = Self>>(iter: I) -> Self {\n\t\titer.fold(Self::from_val(1), |b, x| b * x)\n\
    \t}\n}\n\nimpl<P: Mod> fmt::Debug for Mint<P> {\n\tfn fmt(&self, f: &mut fmt::Formatter)\
    \ -> fmt::Result {\n\t\tself.val.fmt(f)\n\t}\n}\n\nimpl<P: Mod> fmt::Display for\
    \ Mint<P> {\n\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n\t\tself.val.fmt(f)\n\
    \t}\n}\n\nimpl<P: Mod> ZeroOne for Mint<P> {\n\tconst ZERO: Self = Self { val:\
    \ 0, _m: PhantomData };\n\tconst ONE: Self = Self { val: 1, _m: PhantomData };\n\
    }\n\nimpl<P: Mod> Num for Mint<P> {}\n\nimpl<M: Mod> Print for Mint<M> {\n\tfn\
    \ print(w: &mut IO, x: Self) {\n\t\tw.print(x.value());\n\t}\n}\n\nimpl<M: Mod>\
    \ Scan for Mint<M> {\n\tfn scan(io: &mut IO) -> Self {\n\t\tio.scan::<u32>().into()\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/mint.rs
  requiredBy: []
  timestamp: '2020-12-10 17:35:58+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/dfa_test.rs
  - test/src/bin/ntt_mint_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_mint_garner_test.rs
documentation_of: src/mint.rs
layout: document
redirect_from:
- /library/src/mint.rs
- /library/src/mint.rs.html
title: src/mint.rs
---
