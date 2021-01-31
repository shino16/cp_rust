---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bound.rs
    title: src/bound.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
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
  code: "use crate::bound::*;\nuse crate::num::*;\nuse std::collections::VecDeque;\n\
    \npub mod edge;\n\n#[derive(Clone, Copy, Debug)]\npub struct InnerEdge<C: Num\
    \ + Bound> {\n\tpub to: usize,\n\tpub cap: C,\n\trev: usize,\n}\n\n/// highest-label\
    \ preflow-push algorithm with global labeling and gap relabeling\n/// O(V^2 \\\
    sqrt(E))\n#[derive(Clone)]\npub struct Hlpp<C: Num + Bound> {\n\tpub graph: Vec<Vec<InnerEdge<C>>>,\n\
    \theight: Vec<usize>,\n\texcess: Vec<C>,\n\tcount: Vec<usize>,\n\ttodo: Vec<Vec<usize>>,\n\
    \theight_inv: Vec<Vec<usize>>,\n\tidx: Vec<usize>,\n\thighest_active: usize,\n\
    \thighest: usize,\n}\n\nimpl<C: Num + Bound> Hlpp<C> {\n\tpub fn new(len: usize)\
    \ -> Self {\n\t\tSelf {\n\t\t\tgraph: vec![Vec::new(); len],\n\t\t\theight: vec![len;\
    \ len],\n\t\t\texcess: vec![C::ZERO; len],\n\t\t\tcount: vec![0; len * 2],\n\t\
    \t\ttodo: vec![Vec::new(); len * 2],\n\t\t\theight_inv: vec![Vec::new(); len *\
    \ 2],\n\t\t\tidx: vec![!0; len],\n\t\t\thighest_active: 0,\n\t\t\thighest: 0,\n\
    \t\t}\n\t}\n\tpub fn len(&self) -> usize {\n\t\tself.graph.len()\n\t}\n\tpub fn\
    \ from_digraph(graph: &[Vec<(usize, C)>]) -> Self {\n\t\tlet mut ret = Self::new(graph.len());\n\
    \t\tfor (v, adj) in (0..).zip(graph) {\n\t\t\tfor &(w, cap) in adj {\n\t\t\t\t\
    ret.add_edge(v, w, cap);\n\t\t\t}\n\t\t}\n\t\tret\n\t}\n\tpub fn add_edge(&mut\
    \ self, v: usize, w: usize, cap: C) {\n\t\tlet (vidx, widx) = (self.graph[v].len(),\
    \ self.graph[w].len());\n\t\tself.graph[v].push(InnerEdge { to: w, cap, rev: widx\
    \ });\n\t\tself.graph[w].push(InnerEdge { to: v, cap: C::ZERO, rev: vidx });\n\
    \t}\n\tfn push(&mut self, v: usize, idx: usize, init: bool) {\n\t\tlet InnerEdge\
    \ { to, ref mut cap, rev } = self.graph[v][idx];\n\t\tdebug_assert!(self.excess[v]\
    \ > C::ZERO);\n\t\tif !init {\n\t\t\tdebug_assert!(self.height[v] == self.height[to]\
    \ + 1);\n\t\t}\n\t\tif *cap == C::ZERO {\n\t\t\treturn;\n\t\t}\n\t\tlet df = self.excess[v].min(*cap);\n\
    \t\t*cap -= df;\n\t\tself.graph[to][rev].cap += df;\n\t\tself.excess[v] -= df;\n\
    \t\tself.excess[to] += df;\n\t\tif !init && self.excess[to] == df {\n\t\t\tself.todo[self.height[to]].push(to);\n\
    \t\t\tself.highest_active = self.highest_active.max(self.height[to]);\n\t\t}\n\
    \t}\n\tfn change_height(&mut self, v: usize, h0: usize, h1: usize) {\n\t\tself.count[h0]\
    \ -= 1;\n\t\tself.height[v] = h1;\n\t\tself.idx[v] = self.height_inv[h1].len();\n\
    \t\tself.height_inv[h1].push(v);\n\t\tself.count[h1] += 1;\n\t\tdebug_assert!(self.highest_active\
    \ <= h1);\n\t\tself.highest_active = h1;\n\t\tif h1 < self.len() {\n\t\t\tself.highest\
    \ = self.highest.max(h1);\n\t\t}\n\t\tself.todo[h1].push(v);\n\t}\n\tfn relabel(&mut\
    \ self, v: usize, h1: usize) {\n\t\tlet h0 = self.height[v];\n\t\tif self.count[h0]\
    \ == 1 && h0 < self.len() {\n\t\t\tlet len = self.len();\n\t\t\tdebug_assert!(self.highest\
    \ < len);\n\t\t\tfor h in h0..=self.highest {\n\t\t\t\tlet mut drain = std::mem::take(&mut\
    \ self.height_inv[h]);\n\t\t\t\tfor v in drain.drain(..) {\n\t\t\t\t\tself.change_height(v,\
    \ h, len);\n\t\t\t\t}\n\t\t\t\tself.height_inv[h] = drain;\n\t\t\t}\n\t\t\tself.highest\
    \ = h0 - 1;\n\t\t} else {\n\t\t\tif self.idx[v] >= self.height_inv[h0].len() {}\n\
    \t\t\tself.height_inv[h0].swap_remove(self.idx[v]);\n\t\t\tif let Some(&w) = self.height_inv[h0].get(self.idx[v])\
    \ {\n\t\t\t\tself.idx[w] = self.idx[v];\n\t\t\t}\n\t\t\tself.change_height(v,\
    \ h0, h1);\n\t\t}\n\t}\n\tfn discharge(&mut self, v: usize) {\n\t\twhile self.excess[v]\
    \ > C::ZERO {\n\t\t\tlet mut min = !0;\n\t\t\tfor i in 0..self.graph[v].len()\
    \ {\n\t\t\t\tif self.graph[v][i].cap > C::ZERO {\n\t\t\t\t\tif self.height[v]\
    \ > self.height[self.graph[v][i].to] {\n\t\t\t\t\t\tself.push(v, i, false);\n\t\
    \t\t\t\t\tif self.excess[v] == C::ZERO {\n\t\t\t\t\t\t\treturn;\n\t\t\t\t\t\t\
    }\n\t\t\t\t\t} else {\n\t\t\t\t\t\tmin = min.min(self.height[self.graph[v][i].to]);\n\
    \t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\tself.relabel(v, min + 1);\n\t\t}\n\t}\n\
    \tfn init(&mut self, s: usize, t: usize) {\n\t\tself.excess[s] = C::MAX;\n\t\t\
    for i in 0..self.graph[s].len() {\n\t\t\tself.push(s, i, true);\n\t\t}\n\t\tself.height[t]\
    \ = 0;\n\t\tlet mut bfs = VecDeque::new();\n\t\tbfs.push_back(t);\n\t\tlet mut\
    \ h = 0;\n\t\twhile let Some(v) = bfs.pop_front() {\n\t\t\th = self.height[v];\n\
    \t\t\tfor &InnerEdge { to, cap: _, rev } in &self.graph[v] {\n\t\t\t\tif self.height[to]\
    \ == self.len() && self.graph[to][rev].cap > C::ZERO {\n\t\t\t\t\tself.height[to]\
    \ = h + 1;\n\t\t\t\t\tbfs.push_back(to);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tself.highest\
    \ = h;\n\t\tfor v in 0..self.len() {\n\t\t\tlet h = self.height[v];\n\t\t\tself.count[h]\
    \ += 1;\n\t\t\tself.idx[v] = self.height_inv[h].len();\n\t\t\tself.height_inv[h].push(v);\n\
    \t\t\tif self.excess[v] > C::ZERO {\n\t\t\t\tself.todo[h].push(v);\n\t\t\t}\n\t\
    \t}\n\t}\n\tpub fn solve(&mut self, s: usize, t: usize) -> C {\n\t\tif s == t\
    \ {\n\t\t\treturn C::ZERO;\n\t\t}\n\t\tself.init(s, t);\n\t\tself.highest_active\
    \ = self.todo.len();\n\t\twhile self.highest_active > 0 {\n\t\t\tself.highest_active\
    \ -= 1;\n\t\t\twhile let Some(v) = self.todo[self.highest_active].pop() {\n\t\t\
    \t\tif v != s && v != t {\n\t\t\t\t\tself.discharge(v);\n\t\t\t\t}\n\t\t\t}\n\t\
    \t}\n\t\tself.excess[t]\n\t}\n}\n"
  dependsOn:
  - src/bound.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/hlpp.rs
  requiredBy:
  - src/graph/max_flow/hlpp/edge.rs
  timestamp: '2021-01-30 17:33:56+09:00'
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
