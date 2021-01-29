pub use super::*;
use std::marker::PhantomData;

pub struct Iter<'a, T: 'a> {
	head: NonNull<Node<T>>,
	tail: NonNull<Node<T>>,
	len: usize,
	_marker: PhantomData<&'a Node<T>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
		let next_val = unsafe { &*self.head.as_ptr() }.next_val.as_ref()?;
		let res = &next_val.1;
		self.head = next_val.0;
		Some(res)
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl<'a, T: 'a> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
		self.tail = unsafe { self.tail.as_ref() }.prev?;
		Some(&unsafe { &*self.tail.as_ptr() }.next_val.as_ref().unwrap().1)
    }
}
