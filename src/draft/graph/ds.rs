pub use crate::graph::*;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub type Dict<'a, V, T> = <V as MkDict<'a, T>>::Type;

pub trait MkDict<'a, T>: Sized {
	type Type: Index<Self, Output = T> + IndexMut<Self> + 'a;
	fn new(self, v: T) -> Self::Type;
}

// impl Dim for usize {}
impl<'a, T: Clone + 'a> MkDict<'a, T> for usize {
	type Type = Vec<T>;
	fn new(self, v: T) -> Self::Type {
		vec![v; self]
	}
}

// impl<A, B> Dim for (A, B) {}
impl<'a, T: Clone + 'a, A: MkDict<'a, B::Type>, B: MkDict<'a, T>> MkDict<'a, T> for (A, B) {
	type Type = DWrap<'a, A::Type, T>;
	fn new(self, v: T) -> Self::Type {
		DWrap(self.0.new(self.1.new(v)), PhantomData)
	}
}

pub struct DWrap<'a, D, T>(D, PhantomData<&'a T>);

impl<'a, T, A: MkDict<'a, B::Type>, B: MkDict<'a, T>> Index<(A, B)> for DWrap<'a, A::Type, T> {
	type Output = T;
	fn index(&self, (a, b): (A, B)) -> &Self::Output {
		&self.0[a][b]
	}
}

impl<'a, T, A: MkDict<'a, B::Type>, B: MkDict<'a, T>> IndexMut<(A, B)>
	for DWrap<'a, A::Type, T>
{
	fn index_mut(&mut self, (a, b): (A, B)) -> &mut Self::Output {
		&mut self.0[a][b]
	}
}
