---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/util/trait_alias.rs
    title: src/util/trait_alias.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edge.rs
    title: src/graph/max_flow/ford_fulkerson/edge.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edges.rs
    title: src/graph/max_flow/ford_fulkerson/edges.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/max_flow/ford_fulkerson.rs\n"
  code: "use crate::bounded::Bounded;\nuse crate::ds::bitset::*;\npub use crate::num::*;\n\
    \npub mod edge;\n\n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: Num + Bounded>\
    \ {\n    pub to: usize,\n    pub cap: C,\n    rev: usize,\n}\n\n/// O(FE)\n#[derive(Clone)]\n\
    pub struct FordFulkerson<C: Num + Bounded> {\n    pub graph: Vec<Vec<Edge<C>>>,\n\
    }\n\nimpl<C: Num + Bounded> FordFulkerson<C> {\n    pub fn new(len: usize) ->\
    \ Self {\n        Self { graph: vec![Vec::new(); len] }\n    }\n    pub fn len(&self)\
    \ -> usize {\n        self.graph.len()\n    }\n    pub fn from_digraph(graph:\
    \ &[Vec<(usize, C)>]) -> Self {\n        let mut ret = Self::new(graph.len());\n\
    \        for (v, adj) in (0..).zip(graph) {\n            for &(w, cap) in adj\
    \ {\n                ret.add_edge(v, w, cap);\n            }\n        }\n    \
    \    ret\n    }\n    pub fn add_vertex(&mut self) -> usize {\n        self.graph.push(Vec::new());\n\
    \        self.graph.len() - 1\n    }\n    pub fn add_edge(&mut self, v: usize,\
    \ w: usize, cap: C) {\n        if cap != C::ZERO {\n            let (vidx, widx)\
    \ = (self.graph[v].len(), self.graph[w].len());\n            self.graph[v].push(Edge\
    \ { to: w, cap, rev: widx });\n            self.graph[w].push(Edge { to: v, cap:\
    \ C::ZERO, rev: vidx });\n        }\n    }\n    pub fn solve(&mut self, s: usize,\
    \ t: usize) -> C {\n        let mut res = C::ZERO;\n        let mut used = new_bitset(self.graph.len());\n\
    \        loop {\n            used.reset();\n            let f = self.dfs(s, t,\
    \ &mut used, C::MAX);\n            if f == C::ZERO {\n                return res;\n\
    \            }\n            res += f;\n        }\n    }\n    pub fn min_cut(&self)\
    \ -> Vec<(usize, usize)> {\n        let mut res = Vec::new();\n        for v in\
    \ 0..self.len() {\n            for e in &self.graph[v] {\n                if e.cap\
    \ == C::ZERO {\n                    res.push((v, e.to));\n                }\n\
    \            }\n        }\n        res\n    }\n    fn dfs(&mut self, v: usize,\
    \ t: usize, used: &mut [u32], ub: C) -> C {\n        if v == t {\n           \
    \ return ub;\n        }\n        let mut adj = std::mem::take(&mut self.graph[v]);\n\
    \        for &mut Edge { to, ref mut cap, rev } in &mut adj {\n            if\
    \ *cap != C::ZERO && used.set_bit(to) {\n                let df = self.dfs(to,\
    \ t, used, ub.min(*cap));\n                if df != C::ZERO {\n              \
    \      *cap -= df;\n                    self.graph[to][rev].cap += df;\n     \
    \               self.graph[v] = adj;\n                    return df;\n       \
    \         }\n            }\n        }\n        self.graph[v] = adj;\n        C::ZERO\n\
    \    }\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/ds/bitset.rs
  - src/num.rs
  - src/util/trait_alias.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/ford_fulkerson.rs
  requiredBy:
  - src/graph/max_flow/ford_fulkerson/edge.rs
  - src/graph/max_flow/ford_fulkerson/edges.rs
  timestamp: '2021-03-19 19:54:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ford_fulkerson_test.rs
documentation_of: src/graph/max_flow/ford_fulkerson.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/ford_fulkerson.rs
- /library/src/graph/max_flow/ford_fulkerson.rs.html
title: src/graph/max_flow/ford_fulkerson.rs
---
