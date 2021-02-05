pub mod cum;
pub mod fill;
pub mod lcp;
pub mod perm;
pub mod sa;
pub mod sort;

pub fn partition_point<T, F: FnMut(&T) -> bool>(slice: &[T], mut pred: F) -> usize {
	let (mut l, mut r) = (0, slice.len()); // pred(slice[r]) == false
	while l != r {
		let mid = (l + r) / 2;
		let val = unsafe { slice.get_unchecked(mid) };
		if pred(val) {
			l = mid + 1;
		} else {
			r = mid;
		}
	}
	r
}

pub fn lower_bound<T: Ord>(slice: &[T], v: &T) -> usize {
	partition_point(slice, |x| x < v)
}

pub fn upper_bound<T: Ord>(slice: &[T], v: &T) -> usize {
	partition_point(slice, |x| x <= v)
}
