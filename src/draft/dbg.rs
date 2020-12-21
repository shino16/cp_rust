#[cfg(debug_assertions)]
macro_rules! dbg {
	($($x:expr),*) => { std::dbg!($($x),*) }
}

#[cfg(not(debug_assertions))]
macro_rules! dbg {
	($($x:expr),*) => { ($($x),*) }
}
