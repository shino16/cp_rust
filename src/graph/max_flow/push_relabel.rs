use std::collections::VecDeque;

use crate::bound::Bound;
use crate::ds::linked_list::ptr::*;
pub use crate::num::*;

#[derive(Clone, Copy, Debug)]
pub struct Edge<C: INum + Bound> {
	pub to: usize,
	pub cap: C,
	rev: usize,
}

/// O(V^2E)
#[derive(Clone)]
pub struct PushRelabel<C: INum + Bound> {
	pub graph: Vec<Vec<Edge<C>>>,
	height: Vec<usize>,
	excess: Vec<C>,
	count: Vec<usize>,
	list: Vec<Vec<usize>>,
	dlist: Vec<LinkedList<usize>>,
	ptr: Vec<CursorPtr<usize>>,
	highest: usize,
	highest_active: usize,
}

impl<C: INum + Bound> PushRelabel<C> {
	pub fn new(len: usize) -> Self {
		Self {
			graph: vec![Vec::new(); len],
			height: vec![0; len],
			excess: vec![C::ZERO; len],
			count: vec![0; len],
			list: vec![Vec::new(); len + 1],
			dlist: vec![LinkedList::new(); len + 1],
			ptr: vec![CursorPtr::dangling(); len],
			highest: 0,
			highest_active: 0,
		}
	}
	pub fn len(&self) -> usize {
		self.graph.len()
	}
	pub fn from_digraph(graph: &[Vec<(usize, C)>]) -> Self {
		let mut ret = Self::new(graph.len());
		for (v, adj) in (0..).zip(graph) {
			for &(w, cap) in adj {
				ret.add_edge(v, w, cap);
			}
		}
		ret
	}
	pub fn add_edge(&mut self, v: usize, w: usize, cap: C) {
		// dbg!(v, w, cap);
		let (vidx, widx) = (self.graph[v].len(), self.graph[w].len());
		self.graph[v].push(Edge { to: w, cap, rev: widx });
		self.graph[w].push(Edge { to: v, cap: C::ZERO, rev: vidx });
	}
	fn init(&mut self, s: usize, t: usize) {
		self.height[s] = self.len();
		self.count[0] = self.len() - 1;
		self.excess[s] = C::MAX;
		self.excess[t] = -C::MAX;
		for v in 0..self.len() {
			if v != s {
				unsafe {
					self.ptr[v] = self.dlist[0].begin_ptr().insert(v);
				}
			}
		}
		for i in 0..self.graph[s].len() {
			self.push(s, i);
		}
		let len = self.len();
		for e in &mut self.height {
			*e = len;
		}
		self.height[t] = 0;
		for e in &mut self.count {
			*e = 0;
		}
		let mut queue = VecDeque::with_capacity(self.len() + 1);
		queue.push_back(t);
		let mut v = 0;
		self.list.iter_mut().for_each(Vec::clear);
		while let Some(v1) = queue.pop_front() {
			v = v1;
			let h = self.height[v] + 1;
			assert_ne!(h, self.len());
			for &Edge { to, cap: _, rev } in &self.graph[v] {
				if self.height[to] == self.len() && self.graph[to][rev].cap > C::ZERO {
					self.height[to] = h;
					self.count[h] += 1;
					unsafe {
						self.ptr[v] = self.dlist[h].begin_ptr().insert(v);
					}
					if self.excess[v] > C::ZERO {
						self.list[h].push(v);
					}
					queue.push_back(to);
				}
			}
		}
		unsafe {
			self.ptr[t] = self.dlist[0].begin_ptr().insert(t);
		}
		self.highest = self.height[v];
		self.highest_active = self.highest; // may not be active
	}
	fn push(&mut self, v: usize, idx: usize) {
		let Edge { to, ref mut cap, rev } = self.graph[v][idx];
		let df = self.excess[v].min(*cap);
		eprintln!("{} to {} by {}", v, to, df);
		*cap -= df;
		self.graph[to][rev].cap += df;
		self.excess[v] -= df;
		self.excess[to] += df;
		if self.excess[to] > C::ZERO && self.excess[to] <= df {
			self.list[self.height[to]].push(to);
		}
	}
	fn discharge(&mut self, v: usize) {
		dbg!(v);
		let mut h2 = self.len();
		for i in 0..self.graph[v].len() {
			let Edge { to, cap, rev: _ } = self.graph[v][i];
			if cap > C::ZERO {
				if self.height[v] == self.height[to] + 1 {
					self.push(v, i);
					if self.excess[v] == C::ZERO {
						return;
					}
				} else {
					h2 = h2.min(self.height[to] + 1);
				}
			}
		}
		let h = self.height[v];
		if self.count[h] == 1 {
			for i in h..=self.highest {
				if i == 8 && self.height[25] == 8 {}
				for &v in self.dlist[i].iter() {
					self.count[self.height[v]] -= 1;
					self.height[v] = self.len();
				}
				self.dlist[i].clear();
			}
		} else {
			if self.count[h] == 0 {
				dbg!(v, h);
			}
			self.count[h] -= 1;
			unsafe {
				self.ptr[v].remove();
			}
			self.height[v] = h2;
			self.count[h2] += 1;
			unsafe {
				self.ptr[v] = self.dlist[h2].begin_ptr().insert(v);
			}
			if h2 != self.len() {
				self.highest_active = h2;
				self.highest = self.highest.max(h2);
				self.list[h2].push(v);
			}
		}
	}
	pub fn solve(&mut self, s: usize, t: usize) -> C {
		self.init(s, t);
		loop {
			if let Some(v) = self.list[self.highest_active].pop() {
				self.discharge(v);
			} else if self.highest_active > 0 {
				self.highest_active -= 1;
			} else {
				break;
			}
		}
		self.excess[t] + C::MAX
	}
}
