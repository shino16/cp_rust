---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/float.rs
    title: src/float.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/float/conv.rs
    title: src/float/conv.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.4/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/complex.rs\n"
  code: "use crate::float::*;\nuse crate::num::*;\nuse crate::zo::*;\nuse std::fmt::Debug;\n\
    use std::ops::*;\n\n#[derive(Default, Clone, Copy, PartialEq, Eq)]\npub struct\
    \ Complex<T> {\n    pub re: T,\n    pub im: T,\n}\n\nimpl<T: Num> Complex<T> {\n\
    \    pub const ZERO: Self = Self { re: T::ZERO, im: T::ZERO };\n    pub const\
    \ ONE: Self = Self { re: T::ONE, im: T::ZERO };\n    pub fn is_zero(&self) ->\
    \ bool { *self == Self::ZERO }\n    pub fn new(re: T, im: T) -> Self { Self {\
    \ re, im } }\n    pub fn conj(self) -> Self where T: Neg<Output = T> {\n     \
    \   Self::new(self.re, -self.im)\n    }\n}\nimpl Complex<Float> {\n    pub fn\
    \ from_polar(r: Float, theta: Float) -> Self {\n        Self { re: r * theta.cos(),\
    \ im: r * theta.sin() }\n    }\n}\nimpl<T: INum> Neg for Complex<T> {\n    type\
    \ Output = Self;\n    fn neg(self) -> Self::Output { Self::new(self.re.neg(),\
    \ self.im.neg()) }\n}\nimpl<T: Num> Add for Complex<T> {\n    type Output = Self;\n\
    \    fn add(self, rhs: Self) -> Self::Output { Self::new(self.re + rhs.re, self.im\
    \ + rhs.im) }\n}\nimpl<T: Num> Sub for Complex<T> {\n    type Output = Self;\n\
    \    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.re - rhs.re, self.im\
    \ - rhs.im) }\n}\nimpl<T: Num> Mul for Complex<T> {\n    type Output = Self;\n\
    \    fn mul(self, rhs: Self) -> Self::Output {\n        Self::new(self.re * rhs.re\
    \ - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)\n    }\n}\nimpl<T:\
    \ Num> Mul<T> for Complex<T> {\n    type Output = Self;\n    fn mul(self, rhs:\
    \ T) -> Self::Output { Self::new(self.re * rhs, self.im * rhs) }\n}\nimpl<T: Num>\
    \ Div for Complex<T> {\n    type Output = Self;\n    fn div(self, rhs: Self) ->\
    \ Self::Output {\n        Self::new(self.re * rhs.re + self.im * rhs.im, self.im\
    \ * rhs.re - self.re * rhs.im)\n            / (rhs.re * rhs.re + rhs.im * rhs.im)\n\
    \    }\n}\nimpl<T: Num> Div<T> for Complex<T> {\n    type Output = Self;\n   \
    \ fn div(self, rhs: T) -> Self::Output { Self::new(self.re / rhs, self.im / rhs)\
    \ }\n}\nmacro_rules! impl_op_assign {\n    ($(($Rhs:ident, $Op:ident, $op:ident,\
    \ $OpAssign:ident, $op_assign:ident)),* $(,)?) => { $(\n        impl<T: Num> $OpAssign<$Rhs>\
    \ for Complex<T> {\n            fn $op_assign(&mut self, rhs: $Rhs) {\n      \
    \          let x = Self::$op(unsafe { std::ptr::read(self) }, rhs);\n        \
    \        *self = x;\n            }\n        }\n    )* };\n}\nimpl_op_assign!(\n\
    \    (Self, Add, add, AddAssign, add_assign),\n    (Self, Sub, sub, SubAssign,\
    \ sub_assign),\n    (Self, Mul, mul, MulAssign, mul_assign),\n    (T, Mul, mul,\
    \ MulAssign, mul_assign),\n    (Self, Div, div, DivAssign, div_assign),\n    (T,\
    \ Div, div, DivAssign, div_assign),\n);\nimpl<T: ZeroOne> ZeroOne for Complex<T>\
    \ {\n    const ZERO: Self = Self { re: T::ZERO, im: T::ZERO };\n    const ONE:\
    \ Self = Self { re: T::ONE, im: T::ZERO };\n}\nimpl<T: ZeroOne> From<T> for Complex<T>\
    \ {\n    fn from(re: T) -> Self { Self { re, im: T::ZERO } }\n}\nimpl Debug for\
    \ Complex<Float> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        f.write_fmt(format_args!(\"{:.2} + {:.2}i\", self.re, self.im))\n\
    \    }\n}\n"
  dependsOn:
  - src/float.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/complex.rs
  requiredBy:
  - src/float/conv.rs
  timestamp: '2021-04-26 15:43:03+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/complex.rs
layout: document
redirect_from:
- /library/src/complex.rs
- /library/src/complex.rs.html
title: src/complex.rs
---
