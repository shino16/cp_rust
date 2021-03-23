use crate::cast::*;
pub use crate::int::ZeroOne;
use crate::int::*;
use crate::io::*;
use std::marker::PhantomData;
use std::{cmp, fmt, iter, ops};

pub trait Mod: Default + Clone + Copy + PartialEq + Eq {
    const P: u32;
    const K: u32; // -1 / P mod 2^32
    const R2: u32; // 2^64 mod P
}

// montgomery reduction (x -> x / 2^32 mod P)
fn reduce<M: Mod>(x: u64) -> u32 {
    let s = M::K.wrapping_mul(x as u32);
    ((x + s as u64 * M::P as u64) >> 32) as u32
}

macro_rules! def_mod {
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

def_mod!(ModA, 1_000_000_007, 2_226_617_417);
def_mod!(ModB, 998_244_353, 998_244_351);
def_mod!(ModC, 1_012_924_417, 1_012_924_415);
def_mod!(ModD, 924_844_033, 924_844_031);

// modular arithmetics
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

/// mod 1_000_000_007
pub type Gf17 = GfA;

/// mod 998_244_353
pub type Gf99 = GfB;

impl<M: Mod> Gf<M> {
    pub const P: u32 = M::P;
    pub fn new(val: u32) -> Self {
        Gf::from_raw(reduce::<M>(val as u64 * M::R2 as u64))
    }
    fn from_raw(val: u32) -> Self {
        Gf { val, _m: PhantomData }
    }
    pub fn value(self) -> u32 {
        let v = reduce::<M>(self.val as u64);
        if v >= M::P { v - M::P } else { v }
    }
    pub fn grow(self) -> GfGrow<M> {
        GfGrow::from_raw((self.val as u64) << 32)
    }
    pub fn mul_unreduced<T: Into<Self>>(self, rhs: T) -> GfGrow<M> {
        GfGrow::from_raw(self.val as u64 * rhs.into().val as u64)
    }
    pub fn pow<I: Int>(self, k: I) -> Self {
        if self.val == 0 && k.is_zero() {
            return Self::new(1);
        }
        let (mut e, mut k) = (self, k.rem_euclid((M::P - 1).as_()));
        let mut res = Self::ONE;
        while !k.is_zero() {
            if !(k & 1.as_()).is_zero() {
                res *= e;
            }
            e *= e;
            k >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        let (mut a, mut b, mut u, mut v) = (M::P as i32, self.value() as i32, 0, 1);
        while b != 0 {
            let t = a / b;
            a -= t * b;
            u -= t * v;
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut u, &mut v);
        }
        debug_assert_eq!(a, 1);
        if u < 0 {
            debug_assert_eq!(v, M::P as i32);
            u += v;
        }
        Self::new(u as u32)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GfGrow<M: Mod> {
    val: u64,
    _m: PhantomData<M>,
}

impl<M: Mod> GfGrow<M> {
    const MOD: u64 = (M::P as u64) << 32;
    fn from_raw(val: u64) -> Self {
        Self { val, _m: PhantomData }
    }
    pub fn reduce(self) -> Gf<M> {
        Gf::from_raw(reduce::<M>(self.val))
    }
    pub fn value(self) -> u32 {
        self.reduce().value()
    }
}

impl<M: Mod> ops::Add<Self> for GfGrow<M> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<M: Mod> ops::AddAssign<Self> for GfGrow<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= Self::MOD * 2 {
            self.val -= Self::MOD * 2;
        }
    }
}

impl<M: Mod> From<GfGrow<M>> for Gf<M> {
    fn from(v: GfGrow<M>) -> Self {
        v.reduce()
    }
}

impl<M: Mod, I: Int> From<I> for Gf<M> {
    fn from(x: I) -> Self {
        Self::new(x.rem_euclid(M::P.as_()).as_())
    }
}

impl<M: Mod> cmp::PartialEq for Gf<M> {
    fn eq(&self, other: &Self) -> bool {
        let val = |obj: &Gf<M>| {
            if obj.val >= M::P {
                obj.val - M::P
            } else {
                obj.val
            }
        };
        val(self) == val(other)
    }
}

impl<M: Mod> cmp::Eq for Gf<M> {}

impl<M: Mod, T: Into<Gf<M>>> ops::AddAssign<T> for Gf<M> {
    fn add_assign(&mut self, rhs: T) {
        self.val += rhs.into().val;
        if self.val >= M::P * 2 {
            self.val -= M::P * 2;
        }
    }
}
impl<M: Mod, T: Into<Gf<M>>> ops::SubAssign<T> for Gf<M> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.val < rhs.val {
            self.val += M::P * 2;
        }
        self.val -= rhs.val;
    }
}
impl<M: Mod, T: Into<Gf<M>>> ops::MulAssign<T> for Gf<M> {
    fn mul_assign(&mut self, rhs: T) {
        *self = self.mul_unreduced(rhs).reduce();
    }
}
impl<M: Mod, T: Into<Gf<M>>> ops::DivAssign<T> for Gf<M> {
    fn div_assign(&mut self, rhs: T) {
        *self *= rhs.into().inv();
    }
}

macro_rules! impl_binop {
    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(
        impl<M: Mod, T: Into<Gf<M>>> ops::$Op<T> for Gf<M> {
            type Output = Self;
            fn $op(mut self, rhs: T) -> Self {
                ops::$OpAssign::$op_assign(&mut self, rhs); self
            }
        }
    )* };
}

impl_binop!(
    (Add, add, AddAssign, add_assign),
    (Sub, sub, SubAssign, sub_assign),
    (Mul, mul, MulAssign, mul_assign),
    (Div, div, DivAssign, div_assign)
);

impl<M: Mod> ops::Neg for Gf<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Gf::from_raw(M::P * 2 - self.val)
    }
}

impl<M: Mod> iter::Sum for Gf<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |b, x| b + x)
    }
}

impl<M: Mod> iter::Product for Gf<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |b, x| b * x)
    }
}

impl<M: Mod> fmt::Debug for Gf<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value().fmt(f)
    }
}

impl<M: Mod> fmt::Display for Gf<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value().fmt(f)
    }
}

impl<M: Mod> ZeroOne for Gf<M> {
    const ZERO: Self = Self { val: 0, _m: PhantomData };
    const ONE: Self = Self {
        val: M::P.wrapping_neg() % M::P,
        _m: PhantomData,
    };
}

impl<M: Mod> Num for Gf<M> {
    fn wrapping_neg(self) -> Self {
        -self
    }
}

impl<M: Mod> Print for Gf<M> {
    fn print(w: &mut IO, x: Self) {
        w.print(x.value());
    }
}

impl<M: Mod> Scan for Gf<M> {
    fn scan(io: &mut IO) -> Self {
        Self::new(io.scan())
    }
}
