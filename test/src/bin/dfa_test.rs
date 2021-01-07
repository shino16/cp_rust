// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570

use lib::dfa::*;
use lib::io::*;
use lib::mint::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum ZigZagState {
	Initial,
	First(u8),
	Up(u8),
	Down(u8),
}

struct ZigZag;

impl Dfa for ZigZag {
	type Alphabet = u8;
	type State = Option<ZigZagState>;
	fn init(&self) -> Self::State {
		Some(ZigZagState::Initial)
	}
	fn next(&self, s: Self::State, a: Self::Alphabet, _: usize) -> Self::State {
		use ZigZagState::*;
		if let Some(s) = s {
			match s {
				Initial if a == b'0' => Some(Initial),
				Initial => Some(First(a)),
				First(d) if d < a => Some(Up(a)),
				First(d) if d > a => Some(Down(a)),
				Up(d) if d > a => Some(Down(a)),
				Down(d) if d < a => Some(Up(a)),
				_ => None,
			}
		} else {
			None
		}
	}
	fn accept(&self, s: Self::State) -> bool {
		s.is_some()
	}
	fn unsuccessful(&self, s: Self::State) -> bool {
		s.is_none()
	}
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Modx;

impl Mod for Modx {
	const P: u32 = 10000;
	const PHI: u32 = 4000;
}

fn main() {
	let mut io = IO::new();
	let [a0, b]: [&[u8]; 2] = io.scan();
	let m = io.scan();
	let mut a = vec![b'0'; b.len()];
	a[b.len() - a0.len()..].copy_from_slice(&a0);

	let dfa = And(ZigZag, And(MultipleOf(m), And(Leq(&b), Not(Lt(&a)))));
	let alphabet = "0123456789".as_bytes();
	let ans: Mint<Modx> = dfa.count(a.len(), alphabet);
	println!("{}", ans);
}
