use crate::float::*;
use crate::num::*;
use crate::zo::*;
use std::fmt::Debug;
use std::ops::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T: Num> Complex<T> {
    pub fn zero() -> Self { Self { re: T::zero(), im: T::zero() } }
    pub fn one() -> Self { Self { re: T::one(), im: T::zero() } }
    pub fn is_zero(&self) -> bool { *self == Self::zero() }
    pub fn new(re: T, im: T) -> Self { Self { re, im } }
    pub fn conj(self) -> Self where T: Neg<Output = T> {
        Self::new(self.re, -self.im)
    }
}
impl Complex<Float> {
    pub fn from_polar(r: Float, theta: Float) -> Self {
        Self { re: r * theta.cos(), im: r * theta.sin() }
    }
}
impl<T: INum> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self::Output { Self::new(self.re.neg(), self.im.neg()) }
}
impl<T: Num> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self::new(self.re + rhs.re, self.im + rhs.im) }
}
impl<T: Num> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.re - rhs.re, self.im - rhs.im) }
}
impl<T: Num> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)
    }
}
impl<T: Num> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output { Self::new(self.re * rhs, self.im * rhs) }
}
impl<T: Num> Div for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.re * rhs.re + self.im * rhs.im, self.im * rhs.re - self.re * rhs.im)
            / (rhs.re * rhs.re + rhs.im * rhs.im)
    }
}
impl<T: Num> Div<T> for Complex<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output { Self::new(self.re / rhs, self.im / rhs) }
}
macro_rules! impl_op_assign {
    ($(($Rhs:ident, $Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),* $(,)?) => { $(
        impl<T: Num> $OpAssign<$Rhs> for Complex<T> {
            fn $op_assign(&mut self, rhs: $Rhs) {
                let x = Self::$op(unsafe { std::ptr::read(self) }, rhs);
                *self = x;
            }
        }
    )* };
}
impl_op_assign!(
    (Self, Add, add, AddAssign, add_assign),
    (Self, Sub, sub, SubAssign, sub_assign),
    (Self, Mul, mul, MulAssign, mul_assign),
    (T, Mul, mul, MulAssign, mul_assign),
    (Self, Div, div, DivAssign, div_assign),
    (T, Div, div, DivAssign, div_assign),
);
impl<T: ZeroOne> ZeroOne for Complex<T> {
    fn zero() -> Self { Self { re: T::zero(), im: T::zero() } }
    fn one() -> Self { Self { re: T::one(), im: T::zero() } }
}
impl<T: ZeroOne> From<T> for Complex<T> {
    fn from(re: T) -> Self { Self { re, im: T::zero() } }
}
impl Debug for Complex<Float> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.2} + {:.2}i", self.re, self.im))
    }
}
