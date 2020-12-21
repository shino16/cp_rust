use super::*;

pub fn bisect<I: Int, F: FnMut(I) -> bool>(mut l: I, mut r: I, mut pred: F) -> I {
	while l != r {
		let mid = (l + r) >> 1;
		if pred(mid) {
			l = mid + I::ONE;
		} else {
			r = mid;
		}
	}
	r
}
