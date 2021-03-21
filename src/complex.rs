use crate::num::*;
use crate::zo::*;
use std::ops::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T: Num> Complex<T> {
    pub fn new(re: T, im: T) -> Self { Self { re, im } }
    pub fn conj(self) -> Self where T: Neg<Output=T> { Self::new(self.re, -self.im) }
}

impl Complex<f64> {
    pub fn from_polar(r: f64, theta: f64) -> Self {
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
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}
impl<T: Num> Mul<T> for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output { Self::new(self.re * rhs, self.im * rhs) }
}
impl<T: Num> Div for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re + self.im * rhs.im,
            self.im * rhs.re - self.re * rhs.im,
        )
    }
}

macro_rules! impl_op_assign {
    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(
        impl<T: Num> $OpAssign for Complex<T> {
            fn $op_assign(&mut self, rhs: Self) {
                let x = Self::$op(unsafe { std::ptr::read(self) }, rhs);
                *self = x;
            }
        }
    )* };
}

impl_op_assign!(
    (Add, add, AddAssign, add_assign),
    (Sub, sub, SubAssign, sub_assign),
    (Mul, mul, MulAssign, mul_assign),
    (Div, div, DivAssign, div_assign)
);

impl<T: ZeroOne> ZeroOne for Complex<T> {
    const ZERO: Self = Self { re: T::ZERO, im: T::ZERO };
    const ONE: Self = Self { re: T::ONE, im: T::ZERO };
}

impl<T: ZeroOne> From<T> for Complex<T> {
    fn from(re: T) -> Self { Self { re, im: T::ZERO } }
}
