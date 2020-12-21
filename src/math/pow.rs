use crate::int::*;

#[inline(always)]
pub fn pow<T: Num, K: UInt>(mut e: T, mut k: K) -> T {
	let mut res = T::ONE;
	while k != K::ZERO {
		if k & K::ONE != K::ZERO {
			res = res * e;
		}
		e = e * e;
		k >>= 1;
	}
	res
}
