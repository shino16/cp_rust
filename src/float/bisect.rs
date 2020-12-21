use super::*;

pub fn bisect<F: FnMut(Float) -> bool>(
	mut l: Float,
	mut r: Float,
	e: Float,
	mut pred: F,
) -> Float {
	let k = ((r - l) / e).log2() as u32 + 2;
	for _ in 0..k {
		let mid = (l + r) / 2.0;
		if pred(mid) {
			l = mid;
		} else {
			r = mid;
		}
	}
	r
}
