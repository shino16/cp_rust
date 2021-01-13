use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
	fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
		self.0.partial_cmp(&rhs.0).unwrap()
	}
}

impl<T> Deref for Total<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for Total<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
