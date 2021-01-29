---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bound.rs
    title: src/bound.rs
  - icon: ':heavy_check_mark:'
    path: src/ds/bitset.rs
    title: src/ds/bitset.rs
  - icon: ':heavy_check_mark:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':heavy_check_mark:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/max_flow/ford_fulkerson.rs\n"
  code: "use crate::bound::Bound;\nuse crate::ds::bitset::*;\npub use crate::num::*;\n\
    \n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: Num + Bound> {\n\tpub to:\
    \ usize,\n\tpub cap: C,\n\trev: usize,\n}\n\n/// O(FE)\n#[derive(Clone)]\npub\
    \ struct FordFulkerson<C: Num + Bound> {\n\tpub graph: Vec<Vec<Edge<C>>>,\n}\n\
    \nimpl<C: Num + Bound> FordFulkerson<C> {\n\tpub fn new(len: usize) -> Self {\n\
    \t\tSelf { graph: vec![Vec::new(); len] }\n\t}\n\tpub fn len(&self) -> usize {\n\
    \t\tself.graph.len()\n\t}\n\tpub fn from_digraph(graph: &[Vec<(usize, C)>]) ->\
    \ Self {\n\t\tlet mut ret = Self::new(graph.len());\n\t\tfor (v, adj) in (0..).zip(graph)\
    \ {\n\t\t\tfor &(w, cap) in adj {\n\t\t\t\tret.add_edge(v, w, cap);\n\t\t\t}\n\
    \t\t}\n\t\tret\n\t}\n\tpub fn add_vertex(&mut self) -> usize {\n\t\tself.graph.push(Vec::new());\n\
    \t\tself.graph.len() - 1\n\t}\n\tpub fn add_edge(&mut self, v: usize, w: usize,\
    \ cap: C) {\n\t\tlet (vidx, widx) = (self.graph[v].len(), self.graph[w].len());\n\
    \t\tself.graph[v].push(Edge { to: w, cap, rev: widx });\n\t\tself.graph[w].push(Edge\
    \ { to: v, cap: C::ZERO, rev: vidx });\n\t}\n\tpub fn solve(&mut self, s: usize,\
    \ t: usize) -> C {\n\t\tlet mut res = C::ZERO;\n\t\tlet mut used = new_bitset(self.graph.len());\n\
    \t\tloop {\n\t\t\tused.reset();\n\t\t\tlet f = Self::dfs(&mut self.graph, s, t,\
    \ &mut used, C::MAX);\n\t\t\tif f == C::ZERO {\n\t\t\t\treturn res;\n\t\t\t}\n\
    \t\t\tres += f;\n\t\t}\n\t}\n\tfn dfs(graph: &mut Vec<Vec<Edge<C>>>, v: usize,\
    \ t: usize, used: &mut [u32], ub: C) -> C {\n\t\tif v == t {\n\t\t\treturn ub;\n\
    \t\t}\n\t\tlet mut adj = std::mem::take(&mut graph[v]);\n\t\tfor &mut Edge { to,\
    \ ref mut cap, rev } in &mut adj {\n\t\t\tif *cap != C::ZERO && used.modify_bit(to,\
    \ true) {\n\t\t\t\tlet df = Self::dfs(graph, to, t, used, ub.min(*cap));\n\t\t\
    \t\tif df != C::ZERO {\n\t\t\t\t\t*cap -= df;\n\t\t\t\t\tgraph[to][rev].cap +=\
    \ df;\n\t\t\t\t\tgraph[v] = adj;\n\t\t\t\t\treturn df;\n\t\t\t\t}\n\t\t\t}\n\t\
    \t}\n\t\tgraph[v] = adj;\n\t\tC::ZERO\n\t}\n}\n"
  dependsOn:
  - src/bound.rs
  - src/ds/bitset.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/ford_fulkerson.rs
  requiredBy: []
  timestamp: '2021-01-29 12:22:27+09:00'
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
