macro_rules! iprod {
	($head:expr) => {
		$head
	};
	($head:expr, $($tail:expr),*) => (
		$head.flat_map(|e| {
			std::iter::repeat(e).zip(iprod!($($tail),*))
		})
	);
}