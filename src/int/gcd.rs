use super::*;
use crate::bits::*;

pub fn gcd<I: Int + Bits>(a: I, b: I) -> I where I::Unsigned: Bits {
    ugcd(a.abs(), b.abs()).as_()
}

// binary gcd
pub fn ugcd<I: UInt + Bits>(a: I, b: I) -> I {
    #[target_feature(enable = "bmi1")]
    unsafe fn ugcd_impl<I: UInt + Bits>(mut a: I, mut b: I) -> I {
        if a.is_zero() {
            return b;
        } else if b.is_zero() {
            return a;
        }
        let a_shift = a.trailing_zeros();
        a >>= a_shift;
        let b_shift = b.trailing_zeros();
        b >>= b_shift;
        while a != b {
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            b -= a;
            b >>= b.trailing_zeros();
        }
        a << a_shift.min(b_shift)
    }
    unsafe {
        ugcd_impl(a, b)
    }
}

/// (x, y, g) where ax + by = g, x >= 0
pub fn extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I, I) {
    // A = [a, x, y; b, u, v], k = [-1; a0; b0]
    // A'= [a, x, y; 0, u, v] \therefore a0*u + b0*v = 0
    let (mut x, mut y, mut u, mut v) = (I::ONE, I::ZERO, I::ZERO, I::ONE);
    while !b.is_zero() {
        let t = a / b;
        a -= t * b;
        x -= t * u;
        y -= t * v;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut u);
        std::mem::swap(&mut y, &mut v);
    }
    if x < I::ZERO {
        x += u;
        y -= v;
        // debug_assert_eq!(gcd(u, v), I::ONE);
        debug_assert!(x + u >= I::ZERO);
    }
    (x, y, a)
}
