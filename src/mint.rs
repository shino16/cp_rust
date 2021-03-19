pub use crate::def_mod;
pub use crate::zo::ZeroOne;
use std::marker::PhantomData;
use std::str::FromStr;
use std::{fmt, iter, ops};

pub mod conv;
pub mod io;

pub trait Mod: Default + Clone + Copy + PartialEq + Eq {
    const M: u32;
    const PHI: u32;
}

#[macro_export]
macro_rules! def_mod {
    ($name:ident, $modulus:expr) => {
        def_mod!($name, $modulus, $modulus - 1);
    };
    ($name:ident, $modulus:expr, $phi:expr) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $name;
        impl Mod for $name {
            const M: u32 = $modulus;
            const PHI: u32 = $phi;
        }
    };
}

def_mod!(ModA, 1_000_000_007);
def_mod!(ModB, 998_244_353);
def_mod!(ModC, 1_012_924_417);
def_mod!(ModD, 924_844_033);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mint<M: Mod> {
    pub val: u32,
    _m: PhantomData<M>,
}

pub type MintA = Mint<ModA>;
pub type MintB = Mint<ModB>;
pub type MintC = Mint<ModC>;
pub type MintD = Mint<ModD>;
pub type Mint17 = MintA;
pub type Mint99 = MintB;

impl<M: Mod> Mint<M> {
    pub const M: u32 = M::M;
    pub fn new(val: i64) -> Self { Self::from_val(val.rem_euclid(M::M as i64) as u32) }
    pub fn from_val(val: u32) -> Self { Mint { val, _m: PhantomData } }
    pub fn value(self) -> u32 { self.val }
    pub fn pow(self, mut exp: u32) -> Self {
        if self.val == 0 && exp == 0 {
            return Self::from_val(1);
        }
        let mut b = self;
        let mut res = Self::from_val(1);
        while exp != 0 {
            if exp % 2 == 1 {
                res *= b;
            }
            b *= b;
            exp >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self { self.pow(M::PHI - 1) }
    pub fn modulus() -> u32 { M::M }
}

macro_rules! impl_from_int {
    ($(($t:ty: $via:ty)),* $(,)?) => { $(
        impl<M: Mod> From<$t> for Mint<M> {
            fn from(x: $t) -> Self { Self::from_val((x as $via).rem_euclid(M::M as $via) as u32) }
        }
    )* };
}
impl_from_int! {
    (i8: i32), (i16: i32), (i32: i32), (i64: i64), (isize: i64),
    (u8: u32), (u16: u32), (u32: u32), (u64: u64), (usize: u64),
}

impl<M: Mod, T: Into<Mint<M>>> ops::Add<T> for Mint<M> {
    type Output = Self;
    fn add(mut self, rhs: T) -> Self {
        self += rhs;
        self
    }
}
impl<M: Mod, T: Into<Mint<M>>> ops::AddAssign<T> for Mint<M> {
    fn add_assign(&mut self, rhs: T) {
        self.val += rhs.into().val;
        if self.val >= M::M {
            self.val -= M::M;
        }
    }
}
impl<M: Mod> ops::Neg for Mint<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Mint::from_val(if self.val == 0 { 0 } else { M::M - self.val })
    }
}
impl<M: Mod, T: Into<Mint<M>>> ops::Sub<T> for Mint<M> {
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        self -= rhs;
        self
    }
}
impl<M: Mod, T: Into<Mint<M>>> ops::SubAssign<T> for Mint<M> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.val < rhs.val {
            self.val += M::M;
        }
        self.val -= rhs.val;
    }
}
impl<M: Mod, T: Into<Mint<M>>> ops::Mul<T> for Mint<M> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let val = self.val as u64 * rhs.into().val as u64 % M::M as u64;
        Self::from_val(val as u32)
    }
}
impl<M: Mod, T: Into<Mint<M>>> ops::MulAssign<T> for Mint<M> {
    fn mul_assign(&mut self, rhs: T) { *self = *self * rhs; }
}
impl<M: Mod, T: Into<Mint<M>>> ops::Div<T> for Mint<M> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        self /= rhs;
        self
    }
}
impl<M: Mod, T: Into<Mint<M>>> ops::DivAssign<T> for Mint<M> {
    fn div_assign(&mut self, rhs: T) { *self *= rhs.into().pow(M::PHI - 1); }
}
impl<M: Mod> iter::Sum for Mint<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from_val(0), |b, x| b + x)
    }
}
impl<M: Mod> iter::Product for Mint<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from_val(1), |b, x| b * x)
    }
}
impl<M: Mod> fmt::Debug for Mint<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.val.fmt(f) }
}
impl<M: Mod> fmt::Display for Mint<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.val.fmt(f) }
}
impl<M: Mod> FromStr for Mint<M> {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { u32::from_str(s).map(Self::from) }
}
impl<M: Mod> ZeroOne for Mint<M> {
    const ZERO: Self = Self { val: 0, _m: PhantomData };
    const ONE: Self = Self { val: 1, _m: PhantomData };
}
