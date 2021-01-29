use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub mod inner_mut;

#[derive(Debug, PartialEq, PartialOrd, Hash)]
pub struct LinkedList<T> {
	head: NonNull<Node<T>>,
	tail: NonNull<Node<T>>,
	len: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Default)]
pub struct Node<T> {
	prev: Option<NonNull<Node<T>>>,
	next_val: Option<(NonNull<Node<T>>, T)>,
}

impl<T> Node<T> {
	fn new() -> Self {
		Self { prev: None, next_val: None }
	}
}

pub struct Iter<'a, T: 'a> {
	head: NonNull<Node<T>>,
	len: usize,
	_marker: PhantomData<&'a Node<T>>,
}

pub struct IntoIter<T> {
	list: LinkedList<T>,
}

#[derive(Debug)]
pub struct CursorMut<'a, T: 'a> {
	pub at: NonNull<Node<T>>,
	list: &'a mut LinkedList<T>,
}

impl<T> LinkedList<T> {
	pub fn new() -> Self {
		let head = Box::leak(Box::new(Node::new())).into();
		Self { head, tail: head, len: 0 }
	}
	pub fn len(&self) -> usize {
		self.len
	}
	pub fn is_empty(&self) -> bool {
		self.head == self.tail
	}
	pub fn clear(&mut self) {
		*self = Self::new();
	}
	pub fn begin_mut(&mut self) -> CursorMut<'_, T> {
		CursorMut { at: self.head, list: self }
	}
	pub fn end_mut(&mut self) -> CursorMut<'_, T> {
		CursorMut { at: self.tail, list: self }
	}
	pub fn push_front(&mut self, val: T) {
		self.begin_mut().insert(val)
	}
	pub fn push_back(&mut self, val: T) {
		self.end_mut().insert(val)
	}
	pub fn pop_front(&mut self) -> Option<T> {
		self.begin_mut().remove()
	}
	pub fn pop_back(&mut self) -> Option<T> {
		self.end_mut().prev()?.remove()
	}
	pub fn iter(&self) -> Iter<'_, T> {
		Iter { head: self.head, len: self.len, _marker: PhantomData }
	}
}

impl<T> FromIterator<T> for LinkedList<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut res = Self::new();
		for val in iter.into_iter() {
			res.end_mut().insert(val);
		}
		res
	}
}

impl<T> IntoIterator for LinkedList<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter {
		IntoIter { list: self }
	}
}

impl<T: Clone> Clone for LinkedList<T> {
	fn clone(&self) -> Self {
		self.iter().cloned().collect()
	}
}

impl<T> Drop for LinkedList<T> {
	fn drop(&mut self) {
		let mut cursor = self.begin_mut();
		while cursor.remove().is_some() {}
	}
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

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.list.pop_front()
	}
}

impl<'a, T: 'a> Deref for CursorMut<'a, T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		unsafe { &self.at.as_ref().next_val.as_ref().unwrap().1 }
	}
}

impl<'a, T: 'a> DerefMut for CursorMut<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1 }
	}
}

impl<'a, T: 'a> CursorMut<'a, T> {
	pub fn next(&mut self) -> Option<&mut Self> {
		self.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;
		Some(self)
	}
	pub fn prev(&mut self) -> Option<&mut Self> {
		self.at = unsafe { self.at.as_ref() }.prev?;
		Some(self)
	}
	pub fn advance(&mut self, by: isize) -> Option<&mut Self> {
		if by >= 0 {
			for _ in 0..by {
				self.next()?;
			}
		} else {
			for _ in by..0 {
				self.prev()?;
			}
		}
		Some(self)
	}
	pub fn insert(&mut self, val: T) {
		let prev = unsafe { self.at.as_ref() }.prev;
		let new_node = Box::leak(Box::new(Node { prev, next_val: Some((self.at, val)) })).into();
		unsafe { self.at.as_mut() }.prev = Some(new_node);
		if let Some(mut prev) = prev {
			unsafe { prev.as_mut() }.next_val.as_mut().unwrap().0 = new_node;
		} else {
			self.list.head = new_node;
		}
		self.at = new_node;
		self.list.len += 1;
	}
	pub fn remove(&mut self) -> Option<T> {
		if self.at == self.list.tail {
			return None;
		}
		unsafe {
			let node = *Box::from_raw(self.at.as_ptr());
			let (mut next, val) = node.next_val?;
			if let Some(mut prev) = node.prev {
				*next.as_mut().prev.as_mut().unwrap() = prev;
				prev.as_mut().next_val.as_mut().unwrap().0 = next;
			} else {
				next.as_mut().prev = None;
				self.list.head = next;
			}
			self.at = next;
			Some(val)
		}
	}
}
