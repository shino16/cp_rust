// verification-helper: PROBLEM https://yukicoder.me/problems/no/880

use lib::ds::segtree::beats::*;
use lib::int::gcd::*;
use lib::io::*;
use lib::iter::Itertools;

#[derive(Debug, Clone, Copy)]
struct E {
	len: usize,
	sum: u64,
	max: u32,
	lcm: u32,
}

#[derive(Debug, Clone, Copy)]
enum F {
	Asgn(u32),
	Gcd(u32),
	Unit,
}
use F::*;

struct M;
impl Monoid for M {
	type Item = E;
	fn unit(&self) -> Self::Item {
		E { len: 0, sum: 0, max: 0, lcm: 1 }
	}
	fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
		if x.len == 0 {
			y
		} else if y.len == 0 {
			x
		} else {
			E {
				len: x.len + y.len,
				sum: x.sum + y.sum,
				max: x.max.max(y.max),
				lcm: lcm(x.lcm, y.lcm),
			}
		}
	}
}

struct A;
impl Monoid for A {
	type Item = F;
	fn unit(&self) -> Self::Item {
		Unit
	}
	fn op(&self, x: Self::Item, y: Self::Item) -> Self::Item {
		match y {
			Asgn(_) => y,
			Gcd(y) => match x {
				Asgn(a) => Asgn(gcd(a, y)),
				Gcd(x) => Gcd(gcd(x, y)),
				_ => Gcd(y),
			},
			_ => x,
		}
	}
}

fn lcm(x: u32, y: u32) -> u32 {
	let lcm = x as u64 * y as u64 / gcd(x, y) as u64;
	(1 << 30).min(lcm) as u32
}

fn fill(a: u32, len: usize) -> E {
	E { len, sum: a as u64 * len as u64, max: a, lcm: a }
}

fn act(e: E, a: F) -> Option<E> {
	match a {
		Asgn(a) => Some(fill(a, e.len)),
		Gcd(a) =>
			if e.len == 1 {
				Some(fill(gcd(e.max, a), 1))
			} else if e.lcm != 1 << 30 && a % e.lcm == 0 {
				Some(e)
			} else {
				None
			},
		_ => Some(e),
	}
}

fn main() {
	let mut io = IO::new();
	let [n, q]: [usize; 2] = io.scan();
	let a = io
		.scan_iter::<u32>(n)
		.map(|a| E { len: 1, sum: a as u64, max: a, lcm: a })
		.collect_vec();
	let mut st = SegmentTreeBeats::from_slice(&a, M, A, act);
	for _ in 0..q {
		let (c, Usize1(l), r) = io.scan();
		match c {
			1 => {
				st.act_over(l, r, Asgn(io.scan()));
			},
			2 => {
				st.act_over(l, r, Gcd(io.scan()));
			},
			3 => {
				io.println(st.ask(l, r).max);
			},
			_ => {
				io.println(st.ask(l, r).sum);
			},
		}
	}
}
