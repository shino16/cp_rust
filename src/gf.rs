// modular arithmetics

use crate::zo::ZeroOne;
use std::marker::PhantomData;
use std::{cmp, fmt, iter, ops::*, str};

pub mod conv;
pub mod dynamic;
pub mod io;

pub trait Mod: Default + Clone + Copy {
    const P: u32;
    const K: u32; // -1 / P mod 2^32
    const R2: u32; // 2^64 mod P
}

// montgomery reduction (x -> x / 2^32 mod P)
fn reduce<M: Mod>(x: u64) -> u32 {
    let s = M::K.wrapping_mul(x as u32);
    ((x + s as u64 * M::P as u64) >> 32) as u32
}

#[macro_export]
macro_rules! def_prime {
    ($name:ident, $modu:expr, $k:expr) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $name;
        impl Mod for $name {
            const P: u32 = $modu;
            const K: u32 = $k;
            const R2: u32 = ((1_u128 << 64) % $modu) as u32;
        }
    };
}

def_prime!(ModA, 1_000_000_007, 2_226_617_417);
def_prime!(ModB, 998_244_353, 998_244_351);
def_prime!(ModC, 1_012_924_417, 1_012_924_415);
def_prime!(ModD, 924_844_033, 924_844_031);

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct Gf<M: Mod> {
    val: u32,
    _m: PhantomData<M>,
}

pub type GfA = Gf<ModA>;
pub type GfB = Gf<ModB>;
pub type GfC = Gf<ModC>;
pub type GfD = Gf<ModD>;
pub type Gf17 = GfA;
pub type Gf99 = GfB;

impl<M: Mod> Gf<M> {
    pub const P: u32 = M::P;
    pub const ZERO: Self = ZeroOne::ZERO;
    pub const ONE: Self = ZeroOne::ONE;
    pub fn new(val: u32) -> Self { val.into() }
    pub fn zero() -> Self { Self::ZERO }
    pub fn one() -> Self { Self::ONE }
    pub fn is_zero(&self) -> bool { *self == Self::ZERO }
    fn from_raw(val: u32) -> Self { Gf { val, _m: PhantomData } }
    pub fn value(self) -> u32 {
        let v = reduce::<M>(self.val as u64);
        if v >= M::P { v - M::P } else { v }
    }
    pub fn pow(mut self, mut k: u64) -> Self {
        if self.val == 0 && k == 0 { return Self::new(1); }
        k %= (M::P - 1) as u64;
        let mut res = Self::ONE;
        while !k.is_zero() {
            if k % 2 != 0 { res *= self; }
            self *= self; k >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        let (mut a, mut b, mut u, mut v) = (M::P as i32, self.value() as i32, 0, 1);
        while b != 0 {
            let t = a / b;
            a -= t * b; u -= t * v;
            std::mem::swap(&mut a, &mut b); std::mem::swap(&mut u, &mut v);
        }
        debug_assert_eq!(a, 1);
        if u < 0 { debug_assert_eq!(v, M::P as i32); u += v; }
        Self::new(u as u32)
    }
}
impl<M: Mod> From<u32> for Gf<M> {
    fn from(x: u32) -> Self { Gf::from_raw(reduce::<M>(x as u64 * M::R2 as u64)) }
}
macro_rules! impl_from_int {
    ($($t:ty),*) => { $(
        impl<M: Mod> From<$t> for Gf<M> {
            fn from(x: $t) -> Self {
                Gf::from_raw(reduce::<M>(x.rem_euclid(M::P as _) as u64 * M::R2 as u64))
            }
        }
    )* };
}
impl_from_int!(u64, usize, i32, i64, isize);
impl<M: Mod> cmp::PartialEq for Gf<M> {
    fn eq(&self, other: &Self) -> bool {
        let val = |obj: &Gf<M>| {
            if obj.val >= M::P { obj.val - M::P } else { obj.val }
        };
        val(self) == val(other)
    }
}
impl<M: Mod> cmp::Eq for Gf<M> {}
impl<M: Mod, T: Into<Gf<M>>> AddAssign<T> for Gf<M> {
    fn add_assign(&mut self, rhs: T) {
        self.val += rhs.into().val;
        if self.val >= M::P * 2 { self.val -= M::P * 2; }
    }
}
impl<M: Mod, T: Into<Gf<M>>> SubAssign<T> for Gf<M> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.val < rhs.val { self.val += M::P * 2; }
        self.val -= rhs.val;
    }
}
impl<M: Mod, T: Into<Gf<M>>> MulAssign<T> for Gf<M> {
    fn mul_assign(&mut self, rhs: T) {
        self.val = reduce::<M>(self.val as u64 * rhs.into().val as u64);
    }
}
impl<M: Mod, T: Into<Gf<M>>> DivAssign<T> for Gf<M> {
    fn div_assign(&mut self, rhs: T) { *self *= rhs.into().inv(); }
}
macro_rules! impl_binop {
    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(
        impl<M: Mod, T: Into<Gf<M>>> $Op<T> for Gf<M> {
            type Output = Self;
            fn $op(mut self, rhs: T) -> Self { self.$op_assign(rhs); self }
        }
    )* };
}
impl_binop!(
    (Add, add, AddAssign, add_assign),
    (Sub, sub, SubAssign, sub_assign),
    (Mul, mul, MulAssign, mul_assign),
    (Div, div, DivAssign, div_assign)
);
impl<M: Mod> Neg for Gf<M> {
    type Output = Self;
    fn neg(self) -> Self { Gf::from_raw(M::P * 2 - self.val) }
}
impl<M: Mod> iter::Sum for Gf<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ZERO, |b, x| b + x) }
}
impl<M: Mod> iter::Product for Gf<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ONE, |b, x| b * x) }
}
impl<M: Mod> fmt::Debug for Gf<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f) }
}
impl<M: Mod> fmt::Display for Gf<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f) }
}
impl<M: Mod> str::FromStr for Gf<M> {
    type Err = <u32 as str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { u32::from_str(s).map(Self::new) }
}
impl<M: Mod> ZeroOne for Gf<M> {
    const ZERO: Self = Self { val: 0, _m: PhantomData };
    const ONE: Self = Self { val: M::P.wrapping_neg() % M::P, _m: PhantomData };
}
