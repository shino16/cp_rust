pub trait ChMaxMin: Ord + Sized {
	fn chmax(&mut self, v: Self) -> bool {
		if *self < v {
			*self = v;
			true
		} else {
			false
		}
	}
	fn chmin(&mut self, v: Self) -> bool {
		if *self > v {
			*self = v;
			true
		} else {
			false
		}
	}
}

impl<T: Ord + Sized> ChMaxMin for T {}
