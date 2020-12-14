use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Vec2D<T> {
	pub h: usize,
	pub w: usize,
	pub inner: Vec<T>,
}

impl<T: Clone> Vec2D<T> {
	pub fn fill(h: usize, w: usize, v: T) -> Self {
		Self { h, w, inner: vec![v; h * w] }
	}
	pub fn resize_from(h: usize, w: usize, inner: Vec<T>) -> Self {
		debug_assert_eq!(inner.len(), h * w);
		Self { h, w, inner }
	}
}

impl<T> Index<[usize; 2]> for Vec2D<T> {
	type Output = T;
	fn index(&self, [r, c]: [usize; 2]) -> &Self::Output {
		&self.inner[r * self.w + c]
	}
}

impl<T> IndexMut<[usize; 2]> for Vec2D<T> {
	fn index_mut(&mut self, [r, c]: [usize; 2]) -> &mut Self::Output {
		&mut self.inner[r * self.w + c]
	}
}

impl<T> Index<usize> for Vec2D<T> {
	type Output = [T];
	fn index(&self, r: usize) -> &Self::Output {
		&self.inner[r * self.w..(r + 1) * self.w]
	}
}

impl<T> IndexMut<usize> for Vec2D<T> {
	fn index_mut(&mut self, r: usize) -> &mut Self::Output {
		&mut self.inner[r * self.w..(r + 1) * self.w]
	}
}
