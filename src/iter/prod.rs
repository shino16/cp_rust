pub trait ProdIterator: Iterator {
	fn prod<J: IntoIterator>(mut self, other: J) -> Product<Self, J::IntoIter>
	where
		Self: Sized,
		Self::Item: Clone,
		J::IntoIter: Clone,
	{
		let other = other.into_iter();
		Product {
			ae: self.next(),
			a: self,
			b: other.clone(),
			b0: other,
		}
	}
}

impl<I: Iterator> ProdIterator for I {}

pub struct Product<I: Iterator, J> {
	a: I,
	ae: Option<I::Item>,
	b: J,
	b0: J,
}

impl<I: Iterator, J: Iterator + Clone> Iterator for Product<I, J>
where
	I::Item: Clone,
{
	type Item = (I::Item, J::Item);
	fn next(&mut self) -> Option<Self::Item> {
		let be = match self.b.next() {
			None => {
				self.b = self.b0.clone();
				match self.b.next() {
					None => return None,
					Some(e) => {
						self.ae = self.a.next();
						e
					},
				}
			},
			Some(e) => e,
		};
		self.ae.as_ref().map(|ae| (ae.clone(), be))
	}
}
