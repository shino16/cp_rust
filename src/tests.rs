#[cfg(test)]
mod tests {
	mod ds {
		mod linked_list {
			// #[test]
			// fn test_linked_list() {
			// 	use crate::ds::linked_list::inner_mut::*;
			// 	use std::cell::RefCell;
			// 	use std::sync::atomic::{AtomicU32, Ordering};

			// 	static DROP_CNT: AtomicU32 = AtomicU32::new(0);
			// 	#[derive(PartialEq, Eq, Clone, Debug)]
			// 	struct S(u32);
			// 	impl Drop for S {
			// 		fn drop(&mut self) {
			// 			DROP_CNT.fetch_add(1, Ordering::SeqCst);
			// 		}
			// 	}

			// 	let mut v = Vec::new();
			// 	let mut l = LinkedList::new();
			// 	let mut l2 = l.clone();
			// 	let mut cur = l2.begin_mut();
			// 	for n in 0..10 {
			// 		v.push(Box::new(S(n)));
			// 		l.push_back(Box::new(S(n)));
			// 		cur.insert(Box::new(S(n)));
			// 		cur.next().unwrap();
			// 	}
			// 	assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());
			// 	assert_eq!(v, l2.into_iter().collect::<Vec<_>>());

			// 	let l = RefCell::new(l);
			// 	let mut cur = LinkedList::begin_inner_mut(&l);
			// 	cur.advance(7).unwrap().remove();
			// 	v.remove(7);
			// 	assert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());
			// 	cur.advance(-2).unwrap().insert(Box::new(S(100)));
			// 	v.insert(5, Box::new(S(100)));
			// 	assert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());
			// 	let mut cur = LinkedList::end_inner_mut(&l);
			// 	cur.advance(-8).unwrap().remove();
			// 	v.remove(2);
			// 	assert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());
			// 	cur.advance(-2).unwrap();
			// 	assert!(cur.prev().is_none());
			// 	std::mem::drop((v, l));
			// 	assert_eq!(DROP_CNT.load(Ordering::SeqCst), 70);
			// }
			// #[test]
			// fn test_linked_list_ptr() {
			// 	use crate::ds::linked_list::ptr::*;
			// 	use std::sync::atomic::{AtomicU32, Ordering};

			// 	static DROP_CNT: AtomicU32 = AtomicU32::new(0);
			// 	#[derive(PartialEq, Eq, Clone, Debug)]
			// 	struct S(u32);
			// 	impl Drop for S {
			// 		fn drop(&mut self) {
			// 			DROP_CNT.fetch_add(1, Ordering::SeqCst);
			// 		}
			// 	}

			// 	let mut v = Vec::new();
			// 	let mut l = LinkedList::new();
			// 	let mut l2 = l.clone();
			// 	let mut cur = l2.begin_mut();
			// 	for n in 0..10 {
			// 		v.push(Box::new(S(n)));
			// 		l.push_back(Box::new(S(n)));
			// 		cur.insert(Box::new(S(n)));
			// 		cur.next().unwrap();
			// 	}
			// 	assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());
			// 	assert_eq!(v, l2.into_iter().collect::<Vec<_>>());

			// 	let mut cur = l.begin_ptr();
			// 	unsafe { cur.advance(7).unwrap().remove(); }
			// 	v.remove(7);
			// 	assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());
			// 	unsafe { cur.advance(-2).unwrap().insert(Box::new(S(100))); }
			// 	v.insert(5, Box::new(S(100)));
			// 	assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());
			// 	let mut cur = l.end_ptr();
			// 	unsafe { cur.advance(-8).unwrap().remove(); }
			// 	v.remove(2);
			// 	assert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());
			// 	cur.advance(-2).unwrap();
			// 	assert!(cur.prev().is_none());
			// 	for (v, l) in v.iter().zip(l.iter()) {
			// 		assert_eq!(v, l);
			// 	}
			// 	std::mem::drop((v, l));
			// 	assert_eq!(DROP_CNT.load(Ordering::SeqCst), 70);
			// }
		}
	}

	mod fp {
		use crate::fp::*;
		#[test]
		fn test_pow() {
			use crate::rand::xoshiro256plus::*;
			let mut rng = Xoshiro256plus::new();
			assert_eq!(F17::new(2).pow(3), F17::new(8));
			for _ in 0..100 {
				let base: F17 = rng.next().into();
				let k = rng.next() % 100;
				let p = (0..k).map(|_| base).product::<F17>();
				assert_eq!(p, base.pow(k));
			}
		}
		#[test]
		fn test_inv() {
			use crate::rand::xoshiro256plus::*;
			let mut rng = Xoshiro256plus::new();
			for _ in 0..100 {
				let a: F17 = rng.next().into();
				let b = a.inv();
				assert!(a * b == F17::ONE, "{} {}", a, b);
			}
		}
	}

	mod fp_naive {
		use crate::mint::*;
		#[test]
		fn test_mul() {
			use crate::rand::xoshiro256plus::*;
			let mut rng = Xoshiro256plus::new();
			for _ in 0..100 {
				let a = rng.next() as u32 as u64;
				let b = rng.next() as u32 as u64;
				assert_eq!(Mint17::from(a) * b, Mint17::from(a * b));
			}
		}
		#[test]
		fn test_pow() {
			use crate::rand::xoshiro256plus::*;
			let mut rng = Xoshiro256plus::new();
			for _ in 0..100 {
				let base: Mint17 = rng.next().into();
				let k = rng.next() % 100;
				let p = (0..k).map(|_| base).product::<Mint17>();
				assert_eq!(p, base.pow(k as u32));
			}
		}
		#[test]
		fn test_inv() {
			use crate::rand::xoshiro256plus::*;
			let mut rng = Xoshiro256plus::new();
			for _ in 0..100 {
				let a: Mint17 = rng.next().into();
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
			let rhs = vec![(0, b'a'), (0, b'b'), (1, b'a'), (1, b'b'), (2, b'a'), (2, b'b')];
			assert_eq!(lhs, rhs);
		}
	}

	mod num {
		use crate::int::*;
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
			use crate::int::gcd::*;
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

	mod slice {
		mod perm {
			use crate::slice::perm::*;
			#[test]
			fn test_next_permutation() {
				let n = 5;
				let mut a: Vec<_> = (0..n).collect();
				let mut b = a.clone();
				let mut cnt = 0;
				while next_permutation(&mut b) {
					assert!(a < b);
					next_permutation(&mut a);
					cnt += 1;
				}
				assert_eq!(cnt, 5 * 4 * 3 * 2 * 1 - 1);
			}
		}
	}
}
