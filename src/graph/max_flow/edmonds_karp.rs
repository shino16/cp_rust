use crate::bounded::Bounded;
pub use crate::num::*;
use std::collections::VecDeque;

pub mod edge;

#[derive(Clone, Copy, Debug)]
pub struct Edge<C: Num + Bounded> {
    pub to: usize,
    pub cap: C,
    rev: usize,
}

/// O(VE^2)
#[derive(Clone)]
pub struct EdmondsKarp<C: Num + Bounded> {
    pub graph: Vec<Vec<Edge<C>>>,
}

impl<C: Num + Bounded> EdmondsKarp<C> {
    pub fn new(len: usize) -> Self {
        Self { graph: vec![Vec::new(); len] }
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
    pub fn add_vertex(&mut self) -> usize {
        self.graph.push(Vec::new());
        self.graph.len() - 1
    }
    pub fn add_edge(&mut self, v: usize, w: usize, cap: C) {
        let (vidx, widx) = (self.graph[v].len(), self.graph[w].len());
        self.graph[v].push(Edge { to: w, cap, rev: widx });
        self.graph[w].push(Edge { to: v, cap: C::zero(), rev: vidx });
    }
    pub fn solve(&mut self, s: usize, t: usize) -> C {
        let mut res = C::zero();
        let mut track = vec![!0; self.len()];
        let mut togo = VecDeque::new();
        loop {
            for e in &mut track {
                *e = !0;
            }
            togo.clear();
            togo.push_back((s, C::MAX));
            let mut df = C::zero();
            while let Some((v, ub)) = togo.pop_front() {
                for &Edge { to, cap, rev } in &self.graph[v] {
                    if cap != C::zero() && track[to] == !0 {
                        track[to] = rev;
                        if to == t {
                            df = ub.min(cap);
                            break;
                        }
                        togo.push_back((to, ub.min(cap)));
                    }
                }
            }
            if df == C::zero() {
                return res;
            }
            res += df;
            let mut v = t;
            while v != s {
                let &mut Edge { to, ref mut cap, rev } = &mut self.graph[v][track[v]];
                *cap += df;
                self.graph[to][rev].cap -= df;
                v = to;
            }
        }
    }
}
