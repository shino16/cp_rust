#[cfg(debug_assertions)]
macro_rules! dbg {
    () => {
        std::eprintln!("[{}:{}]", std::file!(), std::line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! dbg {
	($($x:expr),*) => { std::convert::identity(($($x),*)) }
}

pub use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Edge<C: Num + Bound> {
	pub from: usize,
	pub to: usize,
	pub cap: C,
	pub flow: C,
}

impl<C: Num + Bound> Hlpp<C> {
	pub fn get_edge(&self, v: usize, idx: usize) -> Edge<C> {
		let e = self.graph[v][idx];
		let rev = self.graph[e.to][e.rev];
		Edge { from: v, to: e.to, cap: e.cap + rev.cap, flow: rev.cap }
	}
	pub fn get_edges_from(&self, v: usize) -> Vec<Edge<C>> {
		(0..self.graph[v].len()).map(|idx| self.get_edge(v, idx)).collect()
	}
	pub fn dbg_edges(&self) {
		for v in 0..self.len() {
			dbg!(v, self.get_edges_from(v));
		}
		dbg!(self.excess[7]);
	}
}
