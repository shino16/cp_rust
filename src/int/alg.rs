use std::marker::PhantomData;

pub use crate::alg::*;
pub use crate::int::*;

#[derive(Default)]
pub struct Addition<N>(PhantomData<N>);

impl<N> Addition<N> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<N: Num> Alg for Addition<N> {
    type Item = N;
}

impl<N: Num> Monoid for Addition<N> {
    fn unit(&self) -> Self::Item {
        N::ZERO
    }
    fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
		x + y
    }
}

impl<N: Num> Group for Addition<N> {
    fn inv(&self, x: Self::Item) -> Self::Item {
        x.wrapping_neg()
    }
}
