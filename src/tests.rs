#[cfg(test)]
mod tests {
	mod fp {
		use crate::fp::*;
		#[test]
		fn test_pow() {
			use crate::rng::*;
			let mut rng = Rng32::new();
			assert_eq!(F17::from(2).pow(3), F17::from(8));
			for _ in 0..100 {
				let base: F17 = rng.gen().into();
				let k = rng.gen() % 100;
				let p = (0..k).map(|_| base).product::<F17>();
				assert_eq!(p, base.pow(k));
			}
		}
		#[test]
		fn test_inv() {
			use crate::rng::*;
			let mut rng = Rng32::new();
			for _ in 0..100 {
				let a: F17 = rng.gen().into();
				let b = a.inv();
				assert!(a * b == F17::ONE, "{} {}", a, b);
			}
		}
	}

	mod fp_naive {
		use crate::mint::*;
		#[test]
		fn test_mul() {
			use crate::rng::*;
			let mut rng = Rng32::new();
			for _ in 0..100 {
				let a = rng.gen() as u64;
				let b = rng.gen() as u64;
				assert_eq!(Mint17::from(a) * b, Mint17::from(a * b));
			}
		}
		#[test]
		fn test_pow() {
			use crate::rng::*;
			let mut rng = Rng32::new();
			for _ in 0..100 {
				let base: Mint17 = rng.gen().into();
				let k = rng.gen() % 100;
				let p = (0..k).map(|_| base).product::<Mint17>();
				assert_eq!(p, base.pow(k.into()));
			}
		}
		#[test]
		fn test_inv() {
			use crate::rng::*;
			let mut rng = Rng32::new();
			for _ in 0..100 {
				let a: Mint17 = rng.gen().into();
				let b = a.inv();
				assert!(a * b == Mint17::ONE, "{} * {} = {}", a, b, a * b);
			}
		}
	}

	mod iter {
		use crate::iter::prod::*;
		use crate::iter::*;
		#[test]
		fn test() {
			let lhs = (0..3).prod(b"ab".to_vec()).collect_vec();
			let rhs = vec![
				(0, b'a'),
				(0, b'b'),
				(1, b'a'),
				(1, b'b'),
				(2, b'a'),
				(2, b'b'),
			];
			assert_eq!(lhs, rhs);
		}
	}

	mod num {
		use crate::num::*;
		#[test]
		fn types() {
			assert_eq!(<i32 as Int>::Signed::ZERO, 0_i32);
			assert_eq!(<i32 as Int>::Unsigned::ZERO, 0_u32);
			assert_eq!(<u32 as Int>::Signed::ZERO, 0_i32);
			assert_eq!(<u32 as Int>::Unsigned::ZERO, 0_u32);
		}
	}

	mod make_vec {
		use crate::make_vec::*;
		#[test]
		fn test() {
			let v = make_vec((3, (5, 8)), "foo");
			assert_eq!(v, vec![vec![vec!["foo"; 8]; 5]; 3]);
		}
	}

	mod math {
		mod gcd {
			use crate::math::gcd::*;
			#[test]
			fn test_gcd() {
				assert_eq!(gcd(0, 0), 0);
				for a in 0..100 {
					for b in 1..100 {
						let g = gcd(a, b);
						for c in g + 1..g {
							assert!(a % c != 0 || b % c != 0);
						}
						assert_eq!(a % g, 0);
						assert_eq!(b % g, 0);
					}
				}
			}
		}
	}
}
