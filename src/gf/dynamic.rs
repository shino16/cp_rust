// modular arithmetics

use crate::zo::ZeroOne;
use crate::math::gcd::ext::extgcd;
use std::marker::PhantomData;
use std::{cmp, fmt, iter, ops::*, str, u32};

pub trait DMod: Default + Clone + Copy {
    fn p() -> u32;  // odd (not necessarily prime)
    fn k() -> u32;  // -1 / P mod 2^32
    fn r2() -> u32; // 2^64 mod P
    unsafe fn set_mod(p: u32);  // may cause data race
}

// montgomery reduction (x -> x / 2^32 mod P)
fn reduce<M: DMod>(x: u64) -> u32 {
    let s = M::k().wrapping_mul(x as u32);
    ((x + s as u64 * M::p() as u64) >> 32) as u32
}

#[macro_export]
macro_rules! def_dyn_mod {
    ($Name:ident, $name:ident) => {
        mod $name {
            use super::*;
            use std::cell::Cell;

            #[derive(Default, Clone, Copy)]
            pub struct $Name;

            struct SyncCell<T>(Cell<T>);
            unsafe impl<T> Sync for SyncCell<T> {}  // not safe

            static P: SyncCell<u32> = SyncCell(Cell::new(1));
            static K: SyncCell<u32> = SyncCell(Cell::new(!0));
            static R2: SyncCell<u32> = SyncCell(Cell::new(0));

            impl DMod for $Name {
                fn p() -> u32 { P.0.get() }
                fn k() -> u32 { K.0.get() }
                fn r2() -> u32 { R2.0.get() }
                unsafe fn set_mod(p: u32) {
                    #[cfg(debug_assertions)]
                    eprintln!("value of {} is set to {}", std::stringify!($Name), p);
                    P.0.set(p);
                    let (g, nk) = extgcd(p as u64, 1 << 32);
                    assert_eq!(g, 1);
                    K.0.set((nk as u32).wrapping_neg());
                    R2.0.set(((p as u64).wrapping_neg() % p as u64) as u32);
                }
            }
        }
        pub use self::$name::$Name;
    };
}

def_dyn_mod!(DefaultMod, default_mod);

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct DynGf<M: DMod = DefaultMod> {
    val: u32,
    _m: PhantomData<*const M>,
}

impl<M: DMod> DynGf<M> {
    pub unsafe fn set_mod(p: u32) { M::set_mod(p); }
    pub const ZERO: Self = Self { val: 0, _m: PhantomData };
    pub fn new(val: u32) -> Self { val.into() }
    pub fn zero() -> Self { Self::from_raw(0) }
    pub fn one() -> Self { 1.into() }
    fn from_raw(val: u32) -> Self { Self { val, _m: PhantomData } }
    pub fn value(self) -> u32 {
        let v = reduce::<M>(self.val as u64);
        if v >= M::p() { v - M::p() } else { v }
    }
    pub fn pow(mut self, mut k: u64) -> Self {
        if self.val == 0 && k == 0 { return Self::new(1); }
        k %= (M::p() - 1) as u64;
        let mut res = Self::one();
        while !k.is_zero() {
            if k % 2 != 0 { res *= self; }
            self *= self; k >>= 1;
        }
        res
    }
    pub fn inv(self) -> Self {
        let (mut a, mut b, mut u, mut v) = (M::p() as i32, self.value() as i32, 0, 1);
        while b != 0 {
            let t = a / b;
            a -= t * b; u -= t * v;
            std::mem::swap(&mut a, &mut b); std::mem::swap(&mut u, &mut v);
        }
        debug_assert_eq!(a, 1);
        if u < 0 { debug_assert_eq!(v, M::p() as i32); u += v; }
        Self::new(u as u32)
    }
}
impl<M: DMod> From<u32> for DynGf<M> {
    fn from(x: u32) -> Self { DynGf::from_raw(reduce::<M>(x as u64 * M::r2() as u64)) }
}
macro_rules! impl_from_int {
    ($($t:ty),*) => { $(
        impl<M: DMod> From<$t> for DynGf<M> {
            fn from(x: $t) -> Self {
                DynGf::from_raw(reduce::<M>(x.rem_euclid(M::p() as _) as u64 * M::r2() as u64))
            }
        }
    )* };
}
impl_from_int!(u64, usize, i32, i64, isize);
impl<M: DMod> cmp::PartialEq for DynGf<M> {
    fn eq(&self, other: &Self) -> bool {
        let val = |obj: &DynGf<M>| {
            if obj.val >= M::p() { obj.val - M::p() } else { obj.val }
        };
        val(self) == val(other)
    }
}
impl<M: DMod> cmp::Eq for DynGf<M> {}
impl<M: DMod, T: Into<DynGf<M>>> AddAssign<T> for DynGf<M> {
    fn add_assign(&mut self, rhs: T) {
        self.val += rhs.into().val;
        if self.val >= M::p() * 2 { self.val -= M::p() * 2; }
    }
}
impl<M: DMod, T: Into<DynGf<M>>> SubAssign<T> for DynGf<M> {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.val < rhs.val { self.val += M::p() * 2; }
        self.val -= rhs.val;
    }
}
impl<M: DMod, T: Into<DynGf<M>>> MulAssign<T> for DynGf<M> {
    fn mul_assign(&mut self, rhs: T) {
        self.val = reduce::<M>(self.val as u64 * rhs.into().val as u64);
    }
}
impl<M: DMod, T: Into<DynGf<M>>> DivAssign<T> for DynGf<M> {
    fn div_assign(&mut self, rhs: T) { *self *= rhs.into().inv(); }
}
macro_rules! impl_binop {
    ($(($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident)),*) => { $(
        impl<M: DMod, T: Into<DynGf<M>>> $Op<T> for DynGf<M> {
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
impl<M: DMod> Neg for DynGf<M> {
    type Output = Self;
    fn neg(self) -> Self { DynGf::from_raw(M::p() * 2 - self.val) }
}
impl<M: DMod> iter::Sum for DynGf<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::zero(), |b, x| b + x) }
}
impl<M: DMod> iter::Product for DynGf<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::one(), |b, x| b * x) }
}
impl<M: DMod> fmt::Debug for DynGf<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f) }
}
impl<M: DMod> fmt::Display for DynGf<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value().fmt(f) }
}
impl<M: DMod> str::FromStr for DynGf<M> {
    type Err = <u32 as str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { u32::from_str(s).map(Self::new) }
}
