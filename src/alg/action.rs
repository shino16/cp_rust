pub use super::*;

pub trait Action {
	type Actor: Monoid;
	type On: Monoid;
	fn act(
		&self,
		on: <Self::On as Alg>::Item,
		actor: <Self::Actor as Alg>::Item,
	) -> <Self::On as Alg>::Item;
}

pub struct ActionImpl<On: Monoid, A: Monoid, F: Fn(On::Item, A::Item) -> On::Item>(
	pub On,
	pub A,
	pub F,
);

impl<On: Monoid, A: Monoid, F: Fn(On::Item, A::Item) -> On::Item> Action
	for ActionImpl<On, A, F>
{
	type Actor = A;
	type On = On;
	fn act(&self, on: On::Item, actor: A::Item) -> On::Item {
		self.2(on, actor)
	}
}
