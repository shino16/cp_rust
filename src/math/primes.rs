use crate::ds::bitset::*;

pub fn primes(n: u32) -> Vec<u32> {
	let n = n as usize;
	let mut prime = new_bitset(n + 1);
	prime.negate();
	for p in 2..=n {
		if prime.get_bit(p) {
			for j in ((p * 2)..=n).step_by(p) {
				prime.set_bit(j, false);
			}
		}
	}
	(2..=n as u32).filter(|&i| prime.get_bit(i as usize)).collect()
}
