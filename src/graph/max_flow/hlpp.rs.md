---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/max_flow/hlpp/edge.rs
    title: src/graph/max_flow/hlpp/edge.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/hlpp_test.rs
    title: test/src/bin/hlpp_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/max_flow/hlpp.rs\n"
  code: "use crate::bounded::*;\nuse crate::num::*;\nuse std::collections::VecDeque;\n\
    \npub mod edge;\n\n#[derive(Clone, Copy, Debug)]\npub struct InnerEdge<C: Num\
    \ + Bounded> {\n    pub to: usize,\n    pub cap: C,\n    rev: usize,\n}\n\n///\
    \ highest-label preflow-push algorithm with global labeling and gap relabeling\n\
    /// O(V^2 \\sqrt(E))\n#[derive(Clone)]\npub struct Hlpp<C: Num + Bounded> {\n\
    \    pub graph: Vec<Vec<InnerEdge<C>>>,\n    height: Vec<usize>,\n    excess:\
    \ Vec<C>,\n    count: Vec<usize>,\n    todo: Vec<Vec<usize>>,\n    height_inv:\
    \ Vec<Vec<usize>>,\n    idx: Vec<usize>,\n    highest_active: usize,\n    highest:\
    \ usize,\n}\n\nimpl<C: Num + Bounded> Hlpp<C> {\n    pub fn new(len: usize) ->\
    \ Self {\n        Self {\n            graph: vec![Vec::new(); len],\n        \
    \    height: vec![len; len],\n            excess: vec![C::ZERO; len],\n      \
    \      count: vec![0; len * 2],\n            todo: vec![Vec::new(); len * 2],\n\
    \            height_inv: vec![Vec::new(); len * 2],\n            idx: vec![!0;\
    \ len],\n            highest_active: 0,\n            highest: 0,\n        }\n\
    \    }\n    pub fn len(&self) -> usize {\n        self.graph.len()\n    }\n  \
    \  pub fn from_digraph(graph: &[Vec<(usize, C)>]) -> Self {\n        let mut ret\
    \ = Self::new(graph.len());\n        for (v, adj) in (0..).zip(graph) {\n    \
    \        for &(w, cap) in adj {\n                ret.add_edge(v, w, cap);\n  \
    \          }\n        }\n        ret\n    }\n    pub fn add_edge(&mut self, v:\
    \ usize, w: usize, cap: C) {\n        let (vidx, widx) = (self.graph[v].len(),\
    \ self.graph[w].len());\n        self.graph[v].push(InnerEdge { to: w, cap, rev:\
    \ widx });\n        self.graph[w].push(InnerEdge { to: v, cap: C::ZERO, rev: vidx\
    \ });\n    }\n    fn push(&mut self, v: usize, idx: usize, init: bool) {\n   \
    \     let InnerEdge { to, ref mut cap, rev } = self.graph[v][idx];\n        debug_assert!(self.excess[v]\
    \ > C::ZERO);\n        if !init {\n            debug_assert!(self.height[v] ==\
    \ self.height[to] + 1);\n        }\n        if *cap == C::ZERO {\n           \
    \ return;\n        }\n        let df = self.excess[v].min(*cap);\n        *cap\
    \ -= df;\n        self.graph[to][rev].cap += df;\n        self.excess[v] -= df;\n\
    \        self.excess[to] += df;\n        if !init && self.excess[to] == df {\n\
    \            self.todo[self.height[to]].push(to);\n            self.highest_active\
    \ = self.highest_active.max(self.height[to]);\n        }\n    }\n    fn change_height(&mut\
    \ self, v: usize, h0: usize, h1: usize) {\n        self.count[h0] -= 1;\n    \
    \    self.height[v] = h1;\n        self.idx[v] = self.height_inv[h1].len();\n\
    \        self.height_inv[h1].push(v);\n        self.count[h1] += 1;\n        debug_assert!(self.highest_active\
    \ <= h1);\n        self.highest_active = h1;\n        if h1 < self.len() {\n \
    \           self.highest = self.highest.max(h1);\n        }\n        self.todo[h1].push(v);\n\
    \    }\n    fn relabel(&mut self, v: usize, h1: usize) {\n        let h0 = self.height[v];\n\
    \        if self.count[h0] == 1 && h0 < self.len() {\n            let len = self.len();\n\
    \            debug_assert!(self.highest < len);\n            for h in h0..=self.highest\
    \ {\n                let mut drain = std::mem::take(&mut self.height_inv[h]);\n\
    \                for v in drain.drain(..) {\n                    self.change_height(v,\
    \ h, len);\n                }\n                self.height_inv[h] = drain;\n \
    \           }\n            self.highest = h0 - 1;\n        } else {\n        \
    \    if self.idx[v] >= self.height_inv[h0].len() {}\n            self.height_inv[h0].swap_remove(self.idx[v]);\n\
    \            if let Some(&w) = self.height_inv[h0].get(self.idx[v]) {\n      \
    \          self.idx[w] = self.idx[v];\n            }\n            self.change_height(v,\
    \ h0, h1);\n        }\n    }\n    fn discharge(&mut self, v: usize) {\n      \
    \  while self.excess[v] > C::ZERO {\n            let mut min = !0;\n         \
    \   for i in 0..self.graph[v].len() {\n                if self.graph[v][i].cap\
    \ > C::ZERO {\n                    if self.height[v] > self.height[self.graph[v][i].to]\
    \ {\n                        self.push(v, i, false);\n                       \
    \ if self.excess[v] == C::ZERO {\n                            return;\n      \
    \                  }\n                    } else {\n                        min\
    \ = min.min(self.height[self.graph[v][i].to]);\n                    }\n      \
    \          }\n            }\n            self.relabel(v, min + 1);\n        }\n\
    \    }\n    fn init(&mut self, s: usize, t: usize) {\n        self.excess[s] =\
    \ C::MAX;\n        for i in 0..self.graph[s].len() {\n            self.push(s,\
    \ i, true);\n        }\n        self.height[t] = 0;\n        let mut bfs = VecDeque::new();\n\
    \        bfs.push_back(t);\n        let mut h = 0;\n        while let Some(v)\
    \ = bfs.pop_front() {\n            h = self.height[v];\n            for &InnerEdge\
    \ { to, cap: _, rev } in &self.graph[v] {\n                if self.height[to]\
    \ == self.len() && self.graph[to][rev].cap > C::ZERO {\n                    self.height[to]\
    \ = h + 1;\n                    bfs.push_back(to);\n                }\n      \
    \      }\n        }\n        self.highest = h;\n        for v in 0..self.len()\
    \ {\n            let h = self.height[v];\n            self.count[h] += 1;\n  \
    \          self.idx[v] = self.height_inv[h].len();\n            self.height_inv[h].push(v);\n\
    \            if self.excess[v] > C::ZERO {\n                self.todo[h].push(v);\n\
    \            }\n        }\n    }\n    pub fn solve(&mut self, s: usize, t: usize)\
    \ -> C {\n        if s == t {\n            return C::ZERO;\n        }\n      \
    \  self.init(s, t);\n        self.highest_active = self.todo.len();\n        while\
    \ self.highest_active > 0 {\n            self.highest_active -= 1;\n         \
    \   while let Some(v) = self.todo[self.highest_active].pop() {\n             \
    \   if v != s && v != t {\n                    self.discharge(v);\n          \
    \      }\n            }\n        }\n        self.excess[t]\n    }\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/hlpp.rs
  requiredBy:
  - src/graph/max_flow/hlpp/edge.rs
  timestamp: '2021-02-08 00:55:24+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/hlpp_test.rs
documentation_of: src/graph/max_flow/hlpp.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/hlpp.rs
- /library/src/graph/max_flow/hlpp.rs.html
title: src/graph/max_flow/hlpp.rs
---
