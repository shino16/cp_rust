use crate::int::*;

#[inline(always)]
pub fn pow<T: Num, K: UInt>(mut e: T, mut k: K) -> T {
    let mut res = T::ONE;
    let two = K::ONE + K::ONE;
    while k != K::ZERO {
        if k % two != K::ZERO {
            res *= e;
        }
        e *= e;
        k /= two;
    }
    res
}
