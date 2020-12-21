pub fn assign_if<T, F: Fn(&T, &T) -> bool>(v: T, var: &mut T, f: F) -> bool {
	if f(&v, var) {
		*var = v;
		true
	} else {
		false
	}
}
