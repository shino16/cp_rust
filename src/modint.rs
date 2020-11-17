// no optimization but works for even modulus

use crate::io::*;
pub use crate::num::ZeroOne as _;
use crate::num::*;
use std::marker::PhantomData;
use std::{fmt, iter, ops};

pub trait Mod: Default + Clone + Copy + PartialEq + Eq {
    const P: u32;
    const PHI: u32;
}

macro_rules! def_mod {
    ($name:ident, $modulus:expr) => {
        def_mod!($name, $modulus, $modulus - 1);
    };
    ($name:ident, $modulus:expr, $phi:expr) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $name;
        impl Mod for $name {
            const P: u32 = $modulus;
            const PHI: u32 = $phi;
        }
    };
}

def_mod!(Mod17, 1_000_000_007);
def_mod!(Mod99, 998_244_353);
def_mod!(Mod10, 1_012_924_417);
def_mod!(Mod92, 924_844_033);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modint<P: Mod> {
    pub val: u32,
    _m: PhantomData<P>,
}

pub type Modint17 = Modint<Mod17>;
pub type Modint99 = Modint<Mod99>;
pub type Modint10 = Modint<Mod10>;
pub type Modint92 = Modint<Mod92>;

impl<P: Mod> Modint<P> {
    pub fn new(val: i64) -> Self { Self::from_val(val.rem_euclid(P::P as i64) as u32) }
    pub fn from_val(val: u32) -> Self { Modint { val, _m: PhantomData } }
    pub fn value(self) -> u32 { self.val }
    pub fn pow(self, mut exp: u32) -> Self {
        if self.val == 0 && exp == 0 {
            return Self::from_val(1);
        }
        let mut e = self;
        let mut res = Self::from_val(1);
        while exp != 0 {
            if exp % 2 == 1 {
                res *= e;
            }
            e *= e;
            exp >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self { self.pow(P::PHI - 1) }
    pub fn modulus() -> u32 { P::P }
}

macro_rules! impl_from_int {
    ($(($ty:ty: $via:ty)),*) => { $(
        impl<P: Mod> From<$ty> for Modint<P> {
            fn from(x: $ty) -> Self {
                Self::from_val((x as $via).rem_euclid(P::P as $via) as u32)
            }
        }
    )* };
}

impl_from_int! {
    (i8: i32), (i16: i32), (i32: i32), (i64: i64), (isize: isize),
    (u8: u32), (u16: u32), (u32: u32), (u64: u64), (usize: usize)
}

impl<P: Mod, T: Into<Modint<P>>> ops::Add<T> for Modint<P> {
    type Output = Self;
    fn add(mut self, rhs: T) -> Self { self += rhs; self }
}

impl<P: Mod, T: Into<Modint<P>>> ops::AddAssign<T> for Modint<P> {
    fn add_assign(&mut self, rhs: T) {
        self.val += rhs.into().val;
        if self.val >= P::P {
            self.val -= P::P;
        }
    }
}

impl<P: Mod> ops::Neg for Modint<P> {
    type Output = Self;
    fn neg(self) -> Self {
        Modint::from_val(match self.val {
            0 => 0,
            s => P::P - s,
        })
    }
}

impl<P: Mod, T: Into<Modint<P>>> ops::Sub<T> for Modint<P> {
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self { self -= rhs; self }
}

impl<P: Mod, T: Into<Modint<P>>> ops::SubAssign<T> for Modint<P> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.val < rhs.val {
            self.val += P::P;
        }
        self.val -= rhs.val;
    }
}

impl<P: Mod, T: Into<Modint<P>>> ops::Mul<T> for Modint<P> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let val = self.val as u64 * rhs.into().val as u64 % P::P as u64;
        Self::from_val(val as u32)
    }
}

impl<P: Mod, T: Into<Modint<P>>> ops::MulAssign<T> for Modint<P> {
    fn mul_assign(&mut self, rhs: T) { *self = *self * rhs; }
}

impl<P: Mod, T: Into<Modint<P>>> ops::Div<T> for Modint<P> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self { self /= rhs; self }
}

impl<P: Mod, T: Into<Modint<P>>> ops::DivAssign<T> for Modint<P> {
    fn div_assign(&mut self, rhs: T) { *self *= rhs.into().pow(P::PHI - 1); }
}

impl<P: Mod> iter::Sum for Modint<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from_val(0), |b, x| b + x)
    }
}

impl<P: Mod> iter::Product for Modint<P> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from_val(1), |b, x| b * x)
    }
}

impl<P: Mod> fmt::Debug for Modint<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.val.fmt(f) }
}

impl<P: Mod> fmt::Display for Modint<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.val.fmt(f) }
}

impl<P: Mod> ZeroOne for Modint<P> {
    const ZERO: Self = Self { val: 0, _m: PhantomData };
    const ONE: Self = Self { val: 1, _m: PhantomData };
}

impl<P: Mod> Num for Modint<P> {}

impl<M: Mod> Print for Modint<M> {
    fn print(w: &mut IO, x: Self) { w.print(x.value()); }
}

impl<M: Mod> Scan for Modint<M> {
    fn scan(io: &mut IO) -> Self { Self::new(io.scan()) }
}
