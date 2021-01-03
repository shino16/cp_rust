pub mod action;
pub mod arith;

pub trait Alg {
	type Item: Copy;
}

pub trait Monoid: Alg {
	fn unit(&self) -> Self::Item;
	fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item;
	fn op_to(&self, y: Self::Item, x: &mut Self::Item) {
		*x = self.op(*x, y);
	}
}

pub trait Group: Monoid {
	fn inv(&self, x: Self::Item) -> Self::Item;
	fn op_inv_to(&self, y: Self::Item, x: &mut Self::Item) {
		*x = self.op(*x, self.inv(y))
	}
}

macro_rules! impl_monoid {
	($target:ty, $($params:tt : $bounds:tt),*) => {
		impl<$($params : $bounds),*> Alg for $target {
			type Item = T;
		}
		impl<$($params : $bounds),*> Monoid for $target {
			fn unit(&self) -> Self::Item {
				(self.0)()
			}
			fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
				(self.1)(x, y)
			}
		}
	};
}

macro_rules! impl_group {
	($target:ty, $($params:tt : $bounds:tt),*) => {
		impl_monoid!($target, $($params : $bounds),*);
		impl<$($params : $bounds),*> Group for $target {
			fn inv(&self, x: Self::Item) -> Self::Item {
				(self.2)(x)
			}
		}
	};
}

pub struct MonoidImpl<T: Copy, Unit: Fn() -> T, Op: Fn(T, T) -> T>(pub Unit, pub Op);
pub struct GroupImpl<T, Unit, Op, Inv>(pub Unit, pub Op, pub Inv)
where
	T: Copy,
	Unit: Fn() -> T,
	Op: Fn(T, T) -> T,
	Inv: Fn(T) -> T;

// help!
impl_monoid!(MonoidImpl<T, Unit, Op>, T: Copy, Unit: (Fn() -> T), Op: (Fn(T, T) -> T));
impl_group!(GroupImpl<T, Unit, Op, Inv>,
			T: Copy, Unit: (Fn() -> T), Op: (Fn(T, T) -> T), Inv: (Fn(T) -> T));
