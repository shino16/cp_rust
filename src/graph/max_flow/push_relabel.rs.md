---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bound.rs
    title: src/bound.rs
  - icon: ':warning:'
    path: src/ds/linked_list.rs
    title: src/ds/linked_list.rs
  - icon: ':warning:'
    path: src/ds/linked_list/ptr.rs
    title: src/ds/linked_list/ptr.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/max_flow/push_relabel.rs\n"
  code: "use std::collections::VecDeque;\n\nuse crate::bound::Bound;\nuse crate::ds::linked_list::ptr::*;\n\
    pub use crate::num::*;\n\n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: INum\
    \ + Bound> {\n\tpub to: usize,\n\tpub cap: C,\n\trev: usize,\n}\n\n/// O(V^2E)\n\
    #[derive(Clone)]\npub struct PushRelabel<C: INum + Bound> {\n\tpub graph: Vec<Vec<Edge<C>>>,\n\
    \theight: Vec<usize>,\n\texcess: Vec<C>,\n\tcount: Vec<usize>,\n\tlist: Vec<Vec<usize>>,\n\
    \tdlist: Vec<LinkedList<usize>>,\n\tptr: Vec<CursorPtr<usize>>,\n\thighest: usize,\n\
    \thighest_active: usize,\n}\n\nimpl<C: INum + Bound> PushRelabel<C> {\n\tpub fn\
    \ new(len: usize) -> Self {\n\t\tSelf {\n\t\t\tgraph: vec![Vec::new(); len],\n\
    \t\t\theight: vec![0; len],\n\t\t\texcess: vec![C::ZERO; len],\n\t\t\tcount: vec![0;\
    \ len],\n\t\t\tlist: vec![Vec::new(); len + 1],\n\t\t\tdlist: vec![LinkedList::new();\
    \ len + 1],\n\t\t\tptr: vec![CursorPtr::dangling(); len],\n\t\t\thighest: 0,\n\
    \t\t\thighest_active: 0,\n\t\t}\n\t}\n\tpub fn len(&self) -> usize {\n\t\tself.graph.len()\n\
    \t}\n\tpub fn from_digraph(graph: &[Vec<(usize, C)>]) -> Self {\n\t\tlet mut ret\
    \ = Self::new(graph.len());\n\t\tfor (v, adj) in (0..).zip(graph) {\n\t\t\tfor\
    \ &(w, cap) in adj {\n\t\t\t\tret.add_edge(v, w, cap);\n\t\t\t}\n\t\t}\n\t\tret\n\
    \t}\n\tpub fn add_edge(&mut self, v: usize, w: usize, cap: C) {\n\t\t// dbg!(v,\
    \ w, cap);\n\t\tlet (vidx, widx) = (self.graph[v].len(), self.graph[w].len());\n\
    \t\tself.graph[v].push(Edge { to: w, cap, rev: widx });\n\t\tself.graph[w].push(Edge\
    \ { to: v, cap: C::ZERO, rev: vidx });\n\t}\n\tfn init(&mut self, s: usize, t:\
    \ usize) {\n\t\tself.height[s] = self.len();\n\t\tself.count[0] = self.len() -\
    \ 1;\n\t\tself.excess[s] = C::MAX;\n\t\tself.excess[t] = -C::MAX;\n\t\tfor v in\
    \ 0..self.len() {\n\t\t\tif v != s {\n\t\t\t\tunsafe {\n\t\t\t\t\tself.ptr[v]\
    \ = self.dlist[0].begin_ptr().insert(v);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tfor i\
    \ in 0..self.graph[s].len() {\n\t\t\tself.push(s, i);\n\t\t}\n\t\tlet len = self.len();\n\
    \t\tfor e in &mut self.height {\n\t\t\t*e = len;\n\t\t}\n\t\tself.height[t] =\
    \ 0;\n\t\tfor e in &mut self.count {\n\t\t\t*e = 0;\n\t\t}\n\t\tlet mut queue\
    \ = VecDeque::with_capacity(self.len() + 1);\n\t\tqueue.push_back(t);\n\t\tlet\
    \ mut v = 0;\n\t\tself.list.iter_mut().for_each(Vec::clear);\n\t\twhile let Some(v1)\
    \ = queue.pop_front() {\n\t\t\tv = v1;\n\t\t\tlet h = self.height[v] + 1;\n\t\t\
    \tassert_ne!(h, self.len());\n\t\t\tfor &Edge { to, cap: _, rev } in &self.graph[v]\
    \ {\n\t\t\t\tif self.height[to] == self.len() && self.graph[to][rev].cap > C::ZERO\
    \ {\n\t\t\t\t\tself.height[to] = h;\n\t\t\t\t\tself.count[h] += 1;\n\t\t\t\t\t\
    unsafe {\n\t\t\t\t\t\tself.ptr[v] = self.dlist[h].begin_ptr().insert(v);\n\t\t\
    \t\t\t}\n\t\t\t\t\tif self.excess[v] > C::ZERO {\n\t\t\t\t\t\tself.list[h].push(v);\n\
    \t\t\t\t\t}\n\t\t\t\t\tqueue.push_back(to);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tunsafe\
    \ {\n\t\t\tself.ptr[t] = self.dlist[0].begin_ptr().insert(t);\n\t\t}\n\t\tself.highest\
    \ = self.height[v];\n\t\tself.highest_active = self.highest; // may not be active\n\
    \t}\n\tfn push(&mut self, v: usize, idx: usize) {\n\t\tlet Edge { to, ref mut\
    \ cap, rev } = self.graph[v][idx];\n\t\tlet df = self.excess[v].min(*cap);\n\t\
    \teprintln!(\"{} to {} by {}\", v, to, df);\n\t\t*cap -= df;\n\t\tself.graph[to][rev].cap\
    \ += df;\n\t\tself.excess[v] -= df;\n\t\tself.excess[to] += df;\n\t\tif self.excess[to]\
    \ > C::ZERO && self.excess[to] <= df {\n\t\t\tself.list[self.height[to]].push(to);\n\
    \t\t}\n\t}\n\tfn discharge(&mut self, v: usize) {\n\t\tdbg!(v);\n\t\tlet mut h2\
    \ = self.len();\n\t\tfor i in 0..self.graph[v].len() {\n\t\t\tlet Edge { to, cap,\
    \ rev: _ } = self.graph[v][i];\n\t\t\tif cap > C::ZERO {\n\t\t\t\tif self.height[v]\
    \ == self.height[to] + 1 {\n\t\t\t\t\tself.push(v, i);\n\t\t\t\t\tif self.excess[v]\
    \ == C::ZERO {\n\t\t\t\t\t\treturn;\n\t\t\t\t\t}\n\t\t\t\t} else {\n\t\t\t\t\t\
    h2 = h2.min(self.height[to] + 1);\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tlet h = self.height[v];\n\
    \t\tif self.count[h] == 1 {\n\t\t\tfor i in h..=self.highest {\n\t\t\t\tif i ==\
    \ 8 && self.height[25] == 8 {}\n\t\t\t\tfor &v in self.dlist[i].iter() {\n\t\t\
    \t\t\tself.count[self.height[v]] -= 1;\n\t\t\t\t\tself.height[v] = self.len();\n\
    \t\t\t\t}\n\t\t\t\tself.dlist[i].clear();\n\t\t\t}\n\t\t} else {\n\t\t\tif self.count[h]\
    \ == 0 {\n\t\t\t\tdbg!(v, h);\n\t\t\t}\n\t\t\tself.count[h] -= 1;\n\t\t\tunsafe\
    \ {\n\t\t\t\tself.ptr[v].remove();\n\t\t\t}\n\t\t\tself.height[v] = h2;\n\t\t\t\
    self.count[h2] += 1;\n\t\t\tunsafe {\n\t\t\t\tself.ptr[v] = self.dlist[h2].begin_ptr().insert(v);\n\
    \t\t\t}\n\t\t\tif h2 != self.len() {\n\t\t\t\tself.highest_active = h2;\n\t\t\t\
    \tself.highest = self.highest.max(h2);\n\t\t\t\tself.list[h2].push(v);\n\t\t\t\
    }\n\t\t}\n\t}\n\tpub fn solve(&mut self, s: usize, t: usize) -> C {\n\t\tself.init(s,\
    \ t);\n\t\tloop {\n\t\t\tif let Some(v) = self.list[self.highest_active].pop()\
    \ {\n\t\t\t\tself.discharge(v);\n\t\t\t} else if self.highest_active > 0 {\n\t\
    \t\t\tself.highest_active -= 1;\n\t\t\t} else {\n\t\t\t\tbreak;\n\t\t\t}\n\t\t\
    }\n\t\tself.excess[t] + C::MAX\n\t}\n}\n"
  dependsOn:
  - src/bound.rs
  - src/ds/linked_list.rs
  - src/ds/linked_list/ptr.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/push_relabel.rs
  requiredBy: []
  timestamp: '2021-01-30 14:00:47+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/max_flow/push_relabel.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/push_relabel.rs
- /library/src/graph/max_flow/push_relabel.rs.html
title: src/graph/max_flow/push_relabel.rs
---
