pub mod conv;
use std::u64;

/// a,b <= modu <= 7.2e18
pub fn modmul64(a: u64, b: u64, modu: u64) -> u64 {
	let (a, b, modu) = (a as i64, b as i64, modu as i64);
	let mut res = a * b - modu * (1.0 / modu as f64 * a as f64 * b as f64) as i64;
	if res < 0 {
		res += modu;
	} else if res >= modu {
		res -= modu;
	}
	res as u64
}

pub fn modpow64(mut e: u64, mut k: u64, modu: u64) -> u64 {
	let mut res = 1;
	while k != 0 {
		if k % 2 != 0 {
			res = modmul64(res, e, modu);
		}
		e = modmul64(e, e, modu);
		k /= 2;
	}
	res
}
