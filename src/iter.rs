pub mod prod;

pub trait Itertools: Iterator {
	fn collect_vec(self) -> Vec<Self::Item>
	where
		Self: Sized,
	{
		self.collect()
	}
}

impl<I: Iterator> Itertools for I {}
