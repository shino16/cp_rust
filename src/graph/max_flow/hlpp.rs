use crate::bound::*;
use crate::num::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub struct InnerEdge<C: Num + Bound> {
	pub to: usize,
	pub cap: C,
	rev: usize,
}

/// highest-label preflow-push algorithm with global labeling and gap-relabeling
/// FIXME: flow may be invalid except around sink
/// O(V^2 \sqrt(E))
#[derive(Clone)]
pub struct Hlpp<C: Num + Bound> {
	pub graph: Vec<Vec<InnerEdge<C>>>,
	height: Vec<usize>,
	excess: Vec<C>,
	count: Vec<usize>,
	todo: Vec<Vec<usize>>,
	height_inv: Vec<Vec<usize>>,
	idx: Vec<usize>,
	highest_active: usize,
	highest: usize,
}

impl<C: Num + Bound> Hlpp<C> {
	pub fn new(len: usize) -> Self {
		Self {
			graph: vec![Vec::new(); len],
			height: vec![len; len],
			excess: vec![C::ZERO; len],
			count: vec![0; len * 2],
			todo: vec![Vec::new(); len + 1],
			height_inv: vec![Vec::new(); len + 1],
			idx: vec![!0; len],
			highest_active: 0,
			highest: 0,
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
		let (vidx, widx) = (self.graph[v].len(), self.graph[w].len());
		self.graph[v].push(InnerEdge { to: w, cap, rev: widx });
		self.graph[w].push(InnerEdge { to: v, cap: C::ZERO, rev: vidx });
	}
	fn push(&mut self, v: usize, idx: usize, init: bool) {
		let InnerEdge { to, ref mut cap, rev } = self.graph[v][idx];
		debug_assert!(self.excess[v] > C::ZERO);
		if !init {
			debug_assert!(self.height[v] == self.height[to] + 1);
		}
		if *cap == C::ZERO {
			return;
		}
		let df = self.excess[v].min(*cap);
		*cap -= df;
		self.graph[to][rev].cap += df;
		self.excess[v] -= df;
		self.excess[to] += df;
		if !init && self.excess[to] > C::ZERO && self.excess[to] <= df {
			self.todo[self.height[to]].push(to);
		}
	}
	fn relabel(&mut self, v: usize, h1: usize) {
		debug_assert!(h1 != 0);
		let h0 = self.height[v];
		if self.count[h0] == 1 {
			let len = self.len();
			for h in h0..=self.highest {
				for v in self.height_inv[h].drain(..) {
					self.count[h] -= 1;
					self.height[v] = len;
				}
			}
			self.highest = h0 - 1;
		} else {
			self.count[h0] -= 1;
			self.height_inv[h0].swap_remove(self.idx[v]);
			if let Some(&w) = self.height_inv[h0].get(self.idx[v]) {
				self.idx[w] = self.idx[v];
			}
			self.height[v] = h1;
			if h1 != self.len() {
				self.idx[v] = self.height_inv[h1].len();
				self.height_inv[h1].push(v);
				self.count[h1] += 1;
				debug_assert!(self.highest_active <= h1);
				self.highest_active = h1;
				self.highest = self.highest.max(h1);
				self.todo[h1].push(v);
			}
		}
	}
	fn discharge(&mut self, v: usize) {
		debug_assert!(self.excess[v] > C::ZERO);
		let mut min = !0;
		for i in 0..self.graph[v].len() {
			if self.graph[v][i].cap > C::ZERO {
				if self.height[v] > self.height[self.graph[v][i].to] {
					self.push(v, i, false);
					if self.excess[v] == C::ZERO {
						return;
					}
				} else {
					min = min.min(self.height[v]);
				}
			}
		}
		self.relabel(v, min + 1);
	}
	fn init(&mut self, s: usize, t: usize) {
		self.excess[s] = C::MAX;
		for i in 0..self.graph[s].len() {
			self.push(s, i, true);
		}
		self.height[t] = 0;
		let mut bfs = VecDeque::new();
		bfs.push_back(t);
		let mut h = 0;
		while let Some(v) = bfs.pop_front() {
			h = self.height[v];
			for &InnerEdge { to, cap: _, rev } in &self.graph[v] {
				if self.height[to] == self.len() && self.graph[to][rev].cap > C::ZERO {
					self.height[to] = h + 1;
					bfs.push_back(to);
				}
			}
		}
		self.highest = h;
		self.highest_active = h;
		for v in 0..self.len() {
			let h = self.height[v];
			self.count[h] += 1;
			self.idx[v] = self.height_inv[h].len();
			self.height_inv[h].push(v);
			if h < self.len() && self.excess[v] > C::ZERO {
				self.todo[h].push(v);
			}
		}
	}
	pub fn solve(&mut self, s: usize, t: usize) -> C {
		if s == t {
			return C::ZERO;
		}
		self.init(s, t);
		self.highest_active += 1;
		while self.highest_active > 0 {
			self.highest_active -= 1;
			while let Some(v) = self.todo[self.highest_active].pop() {
				if v != s && v != t {
					self.discharge(v);
				}
			}
		}
		self.excess[t]
	}
}
