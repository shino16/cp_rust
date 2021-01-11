pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
	if a.len() <= 1 {
		return false;
	}
	let mut k = a.len() - 1;
	while k != 0 && a[k - 1] >= a[k] {
		k -= 1;
	}
	if k == 0 {
		a.reverse();
		return false;
	}
	k -= 1;
	let mut l = a.len() - 1;
	while a[k] >= a[l] {
		l -= 1;
	}
	a.swap(k, l);
	a[k + 1..].reverse();
	true
}
