use crate::as_int::*;
pub use crate::int::ZeroOne as _;
use crate::int::*;
use crate::io::*;
use std::marker::PhantomData;
use std::{fmt, iter, ops};

pub trait Mod: Default + Clone + Copy + PartialEq + Eq {
    const M: u32;
    const PHI: u32;
    const K: u32; // -1 / M mod 2^32
    const R2: u32; // 2^64 mod M
}

// montgomery reduction (x -> x / 2^32 mod M)
fn redc<M: Mod>(x: u64) -> u32 {
    let s = M::K.wrapping_mul(x as u32);
    let t = x + s as u64 * M::M as u64;
    let u = (t >> 32) as u32;
    if u < M::M { u } else { u - M::M }
}

macro_rules! def_mod {
    ($name:ident, $modu:expr, $k:expr, $r2:expr) => {
        def_mod!($name, $modu, $modu - 1, $k, $r2);
    };
    ($name:ident, $modu:expr, $phi:expr, $k:expr, $r2:expr) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $name;
        impl Mod for $name {
            const M: u32 = $modu;
            const PHI: u32 = $phi;
            const K: u32 = $k;
            const R2: u32 = $r2;
        }
    };
}

def_mod!(Mod17, 1_000_000_007, 2_226_617_417, 582_344_008);
def_mod!(Mod99, 998_244_353, 998_244_351, 932_051_910);
def_mod!(Mod10, 1_012_924_417, 1_012_924_415, 818_184_550);
def_mod!(Mod92, 924_844_033, 924844031, 404_973_864);

// modular arithmetics
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fp<M: Mod> {
    val: u32,
    _m: PhantomData<M>,
}

pub type Fp17 = Fp<Mod17>;
pub type Fp99 = Fp<Mod99>;
pub type Fp10 = Fp<Mod10>;
pub type Fp92 = Fp<Mod92>;

impl<M: Mod> Fp<M> {
    pub fn new(val: u32) -> Self {
        Fp::from_raw(redc::<M>(val as u64 * M::R2 as u64))
    }
    fn from_raw(val: u32) -> Self {
        Fp { val, _m: PhantomData }
    }
    pub fn value(self) -> u32 {
        redc::<M>(self.val as u64)
    }
    pub fn grow(self) -> FpGrow<M> {
        FpGrow::from_raw((self.val as u64) << 32)
    }
    pub fn mul_unreduced<T: Into<Self>>(self, rhs: T) -> FpGrow<M> {
        FpGrow::from_raw(self.val as u64 * rhs.into().val as u64)
    }
    pub fn pow<I>(self, exp: I) -> Self
    where
        I: UInt + AsInt<u64>,
        u32: AsInt<I>,
    {
        if self.val == 0 && exp.is_zero() {
            return Self::new(1);
        }
        let (mut base, mut exp) = (self.val as u64, exp % M::PHI.as_());
        let mut res = 1;
        let m = M::M as u64;
        while !exp.is_zero() {
            if !(exp % 2.as_()).is_zero() {
                res = res * base % m;
            }
            base = base * base % m;
            exp >>= 1;
        }
        res.into()
    }
    pub fn inv(self) -> Self {
        let (mut a, mut b, mut u, mut v) = (self.value() as i32, M::M as i32, 1, 0);
        while b != 0 {
            let t = a / b;
            a -= t * b;
            u -= t * v;
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut u, &mut v);
        }
        if u < 0 {
            u += M::M as i32;
        }
        Self::new(u as u32)
    }
    pub fn modu() -> u32 {
        M::M
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct FpGrow<M: Mod> {
    val: u64,
    _m: PhantomData<M>,
}

impl<M: Mod> FpGrow<M> {
    fn from_raw(val: u64) -> Self {
        Self { val, _m: PhantomData }
    }
    pub fn reduce(self) -> Fp<M> {
        Fp::from_raw(redc::<M>(self.val))
    }
    pub fn value(self) -> u32 {
        self.reduce().value()
    }
}

impl<M: Mod> From<FpGrow<M>> for Fp<M> {
    fn from(v: FpGrow<M>) -> Self {
        v.reduce()
    }
}

macro_rules! impl_from_int {
    ($(($ty:ty: $via:ty)),*) => { $(
        impl<M: Mod> From<$ty> for Fp<M> {
            fn from(x: $ty) -> Self {
                Self::new((x as $via).rem_euclid(M::M as $via) as u32)
            }
        }
        impl<M: Mod> From<$ty> for FpGrow<M> {
            fn from(x: $ty) -> Self {
                Fp::from(x).grow()
            }
        }
    )* };
}

impl_from_int! {
    (i8: i32), (i16: i32), (i32: i32), (i64: i64), (isize: isize),
    (u8: u32), (u16: u32), (u32: u32), (u64: u64), (usize: usize)
}

impl<M: Mod, T: Into<Fp<M>>> ops::Add<T> for Fp<M> {
    type Output = Self;
    fn add(mut self, rhs: T) -> Self {
        self += rhs;
        self
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::AddAssign<T> for Fp<M> {
    fn add_assign(&mut self, rhs: T) {
        self.val += rhs.into().val;
        if self.val >= M::M {
            self.val -= M::M;
        }
    }
}
impl<M: Mod> ops::Neg for Fp<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Fp::from_raw(M::M - self.val)
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::Sub<T> for Fp<M> {
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        self -= rhs;
        self
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::SubAssign<T> for Fp<M> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.val < rhs.val {
            self.val += M::M;
        }
        self.val -= rhs.val;
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::Mul<T> for Fp<M> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        self.mul_unreduced(rhs).reduce()
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::MulAssign<T> for Fp<M> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::Div<T> for Fp<M> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self {
        self /= rhs;
        self
    }
}
impl<M: Mod, T: Into<Fp<M>>> ops::DivAssign<T> for Fp<M> {
    fn div_assign(&mut self, rhs: T) {
        *self *= rhs.into().inv();
    }
}

impl<M: Mod> iter::Sum for Fp<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from_raw(0), |b, x| b + x)
    }
}
impl<M: Mod> iter::Product for Fp<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from_raw(1), |b, x| b * x)
    }
}

impl<M: Mod> fmt::Debug for Fp<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value().fmt(f)
    }
}
impl<M: Mod> fmt::Display for Fp<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value().fmt(f)
    }
}

impl<M: Mod> ZeroOne for Fp<M> {
    const ZERO: Self = Self { val: 0, _m: PhantomData };
    const ONE: Self = Self {
        val: M::M.wrapping_neg() % M::M,
        _m: PhantomData,
    };
}

impl<M: Mod> Print for Fp<M> {
    fn print(w: &mut IO, x: Self) {
        w.print(x.value());
    }
}
impl<M: Mod> Scan for Fp<M> {
    fn scan(io: &mut IO) -> Self {
        Self::new(io.scan())
    }
}

impl<M: Mod> ops::Add<Self> for FpGrow<M> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}
impl<M: Mod> ops::AddAssign<Self> for FpGrow<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
    }
}
