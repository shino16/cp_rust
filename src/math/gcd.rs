type Int = i32;
type UInt = u64;

pub fn gcd(a: Int, b: Int) -> Int {
	ugcd(a.abs() as _, b.abs() as _) as _
}

// binary gcd
pub fn ugcd(a: UInt, b: UInt) -> UInt {
	#[target_feature(enable = "bmi1")]
	unsafe fn ugcd_impl(mut a: UInt, mut b: UInt) -> UInt {
		if a == 0 {
			return b;
		} else if b == 0 {
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
pub fn extgcd(mut a: Int, mut b: Int) -> (Int, Int, Int) {
	// A = [a, x, y; b, u, v], k = [-1; a0; b0]
	// A'= [a, x, y; 0, u, v] \therefore a0*u + b0*v = 0
	let (mut x, mut y, mut u, mut v) = (1, 0, 0, 1);
	while b != 0 {
		let t = a / b;
		a -= t * b;
		x -= t * u;
		y -= t * v;
		std::mem::swap(&mut a, &mut b);
		std::mem::swap(&mut x, &mut u);
		std::mem::swap(&mut y, &mut v);
	}
	if x < 0 {
		x += u;
		y -= v;
		debug_assert_eq!(gcd(u, v), 1);
		debug_assert!(x + u >= 0);
	}
	(x, y, a)
}
