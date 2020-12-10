use crate::int::*;

// binary gcd
pub fn gcd<I: Int>(mut a: I, mut b: I) -> I {
	if a.is_zero() {
		return b;
	} else if b.is_zero() {
		return a;
	}
	let a_shift = a.trailing_zeros();
	a >>= a_shift;
	let b_shift = b.trailing_zeros();
	b >>= b_shift;
	let shift = a_shift.min(b_shift);
	loop {
		if a > b {
			std::mem::swap(&mut a, &mut b);
		}
		b -= a;
		if b.is_zero() {
			return a << shift;
		}
		b >>= b.trailing_zeros();
	}
}

// (x, y, g) where ax + by = g
pub fn extgcd<I: IInt>(mut a: I, mut b: I) -> (I, I, I) {
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
	(x, y, a)
}
