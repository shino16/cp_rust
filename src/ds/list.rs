use std::iter::FromIterator;
use std::rc::Rc;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct List<T>(Option<Rc<(T, List<T>)>>);

impl<T> Default for List<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> List<T> {
	pub fn new() -> Self {
		Self(None)
	}
	pub fn push(&mut self, val: T) {
		unsafe {
			std::ptr::write(self, Self(Some(Rc::new((val, std::ptr::read(self))))));
		}
	}
	pub fn cons(self, val: T) -> Self {
		Self(Some(Rc::new((val, self))))
	}
	pub fn head(&self) -> Option<&T> {
		self.0.as_deref().map(|(hd, _)| hd)
	}
	pub fn tail(&self) -> Option<&Self> {
		self.0.as_deref().map(|(_, tl)| tl)
	}
	pub fn from_rev_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		iter.into_iter().fold(Self::new(), |es, e| es.cons(e))
	}
}

pub struct Iter<'a, T>(&'a List<T>);

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		match (self.0).0.as_deref() {
			None => None,
			Some(&(ref hd, ref tl)) => {
				*self = Self(tl);
				Some(hd)
			},
		}
	}
}

impl<T> FromIterator<T> for List<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut iter = iter.into_iter();
		let val = iter.next();
		Self(val.map(|val| Rc::new((val, iter.collect()))))
	}
}
