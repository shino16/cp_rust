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
    path: src/graph/max_flow/edmonds_karp/edge.rs
    title: src/graph/max_flow/edmonds_karp/edge.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/edmonds_karp_test.rs
    title: test/src/bin/edmonds_karp_test.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/max_flow/edmonds_karp.rs\n"
  code: "use crate::bounded::Bounded;\npub use crate::num::*;\nuse std::collections::VecDeque;\n\
    \npub mod edge;\n\n#[derive(Clone, Copy, Debug)]\npub struct Edge<C: Num + Bounded>\
    \ {\n\tpub to: usize,\n\tpub cap: C,\n\trev: usize,\n}\n\n/// O(VE^2)\n#[derive(Clone)]\n\
    pub struct EdmondsKarp<C: Num + Bounded> {\n\tpub graph: Vec<Vec<Edge<C>>>,\n\
    }\n\nimpl<C: Num + Bounded> EdmondsKarp<C> {\n\tpub fn new(len: usize) -> Self\
    \ {\n\t\tSelf { graph: vec![Vec::new(); len] }\n\t}\n\tpub fn len(&self) -> usize\
    \ {\n\t\tself.graph.len()\n\t}\n\tpub fn from_digraph(graph: &[Vec<(usize, C)>])\
    \ -> Self {\n\t\tlet mut ret = Self::new(graph.len());\n\t\tfor (v, adj) in (0..).zip(graph)\
    \ {\n\t\t\tfor &(w, cap) in adj {\n\t\t\t\tret.add_edge(v, w, cap);\n\t\t\t}\n\
    \t\t}\n\t\tret\n\t}\n\tpub fn add_vertex(&mut self) -> usize {\n\t\tself.graph.push(Vec::new());\n\
    \t\tself.graph.len() - 1\n\t}\n\tpub fn add_edge(&mut self, v: usize, w: usize,\
    \ cap: C) {\n\t\tlet (vidx, widx) = (self.graph[v].len(), self.graph[w].len());\n\
    \t\tself.graph[v].push(Edge { to: w, cap, rev: widx });\n\t\tself.graph[w].push(Edge\
    \ { to: v, cap: C::ZERO, rev: vidx });\n\t}\n\tpub fn solve(&mut self, s: usize,\
    \ t: usize) -> C {\n\t\tlet mut res = C::ZERO;\n\t\tlet mut track = vec![!0; self.len()];\n\
    \t\tlet mut togo = VecDeque::new();\n\t\tloop {\n\t\t\tfor e in &mut track {\n\
    \t\t\t\t*e = !0;\n\t\t\t}\n\t\t\ttogo.clear();\n\t\t\ttogo.push_back((s, C::MAX));\n\
    \t\t\tlet mut df = C::ZERO;\n\t\t\twhile let Some((v, ub)) = togo.pop_front()\
    \ {\n\t\t\t\tfor &Edge { to, cap, rev } in &self.graph[v] {\n\t\t\t\t\tif cap\
    \ != C::ZERO && track[to] == !0 {\n\t\t\t\t\t\ttrack[to] = rev;\n\t\t\t\t\t\t\
    if to == t {\n\t\t\t\t\t\t\tdf = ub.min(cap);\n\t\t\t\t\t\t\tbreak;\n\t\t\t\t\t\
    \t}\n\t\t\t\t\t\ttogo.push_back((to, ub.min(cap)));\n\t\t\t\t\t}\n\t\t\t\t}\n\t\
    \t\t}\n\t\t\tif df == C::ZERO {\n\t\t\t\treturn res;\n\t\t\t}\n\t\t\tres += df;\n\
    \t\t\tlet mut v = t;\n\t\t\twhile v != s {\n\t\t\t\tlet &mut Edge { to, ref mut\
    \ cap, rev } = &mut self.graph[v][track[v]];\n\t\t\t\t*cap += df;\n\t\t\t\tself.graph[to][rev].cap\
    \ -= df;\n\t\t\t\tv = to;\n\t\t\t}\n\t\t}\n\t}\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/edmonds_karp.rs
  requiredBy:
  - src/graph/max_flow/edmonds_karp/edge.rs
  timestamp: '2021-02-03 06:45:01+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/edmonds_karp_test.rs
documentation_of: src/graph/max_flow/edmonds_karp.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/edmonds_karp.rs
- /library/src/graph/max_flow/edmonds_karp.rs.html
title: src/graph/max_flow/edmonds_karp.rs
---
