use crate::int::*;

#[inline(always)]
pub fn pow<T: Num, K: UInt>(mut e: T, mut k: K) -> T {
    let mut res = T::one();
    let two = K::one() + K::one();
    while k != K::zero() {
        if k % two != K::zero() {
            res *= e;
        }
        e *= e;
        k /= two;
    }
    res
}
