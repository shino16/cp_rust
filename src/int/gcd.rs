use super::*;
use crate::bits::*;

pub mod ext;

pub fn gcd<I: Int + Bits>(a: I, b: I) -> I
where
    I::Unsigned: Bits,
{
    ugcd(a.abs(), b.abs()).as_()
}

// binary gcd
pub fn ugcd<I: UInt + Bits>(mut a: I, mut b: I) -> I {
    if a.is_zero() {
        return b;
    } else if b.is_zero() {
        return a;
    }
    let ak = a.trailing_zeros();
    a >>= ak;
    let bk = b.trailing_zeros();
    b >>= bk;
    while a != b {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        b >>= b.trailing_zeros();
    }
    a << ak.min(bk)
}
