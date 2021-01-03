pub fn assign_if<T, F: Fn(&T, &T) -> bool>(v: T, var: &mut T, f: F) -> bool {
	if f(&v, var) {
		*var = v;
		true
	} else {
		false
	}
}

pub fn chmin<T: Ord>(v: T, var: &mut T) -> bool {
	assign_if(v, var, |x, y| x < y)
}
