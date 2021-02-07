---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/bounded.rs
    title: src/bounded.rs
  - icon: ':question:'
    path: src/num.rs
    title: src/num.rs
  - icon: ':question:'
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
    \ {\n    pub to: usize,\n    pub cap: C,\n    rev: usize,\n}\n\n/// O(VE^2)\n\
    #[derive(Clone)]\npub struct EdmondsKarp<C: Num + Bounded> {\n    pub graph: Vec<Vec<Edge<C>>>,\n\
    }\n\nimpl<C: Num + Bounded> EdmondsKarp<C> {\n    pub fn new(len: usize) -> Self\
    \ {\n        Self { graph: vec![Vec::new(); len] }\n    }\n    pub fn len(&self)\
    \ -> usize {\n        self.graph.len()\n    }\n    pub fn from_digraph(graph:\
    \ &[Vec<(usize, C)>]) -> Self {\n        let mut ret = Self::new(graph.len());\n\
    \        for (v, adj) in (0..).zip(graph) {\n            for &(w, cap) in adj\
    \ {\n                ret.add_edge(v, w, cap);\n            }\n        }\n    \
    \    ret\n    }\n    pub fn add_vertex(&mut self) -> usize {\n        self.graph.push(Vec::new());\n\
    \        self.graph.len() - 1\n    }\n    pub fn add_edge(&mut self, v: usize,\
    \ w: usize, cap: C) {\n        let (vidx, widx) = (self.graph[v].len(), self.graph[w].len());\n\
    \        self.graph[v].push(Edge { to: w, cap, rev: widx });\n        self.graph[w].push(Edge\
    \ { to: v, cap: C::ZERO, rev: vidx });\n    }\n    pub fn solve(&mut self, s:\
    \ usize, t: usize) -> C {\n        let mut res = C::ZERO;\n        let mut track\
    \ = vec![!0; self.len()];\n        let mut togo = VecDeque::new();\n        loop\
    \ {\n            for e in &mut track {\n                *e = !0;\n           \
    \ }\n            togo.clear();\n            togo.push_back((s, C::MAX));\n   \
    \         let mut df = C::ZERO;\n            while let Some((v, ub)) = togo.pop_front()\
    \ {\n                for &Edge { to, cap, rev } in &self.graph[v] {\n        \
    \            if cap != C::ZERO && track[to] == !0 {\n                        track[to]\
    \ = rev;\n                        if to == t {\n                            df\
    \ = ub.min(cap);\n                            break;\n                       \
    \ }\n                        togo.push_back((to, ub.min(cap)));\n            \
    \        }\n                }\n            }\n            if df == C::ZERO {\n\
    \                return res;\n            }\n            res += df;\n        \
    \    let mut v = t;\n            while v != s {\n                let &mut Edge\
    \ { to, ref mut cap, rev } = &mut self.graph[v][track[v]];\n                *cap\
    \ += df;\n                self.graph[to][rev].cap -= df;\n                v =\
    \ to;\n            }\n        }\n    }\n}\n"
  dependsOn:
  - src/bounded.rs
  - src/num.rs
  - src/zo.rs
  isVerificationFile: false
  path: src/graph/max_flow/edmonds_karp.rs
  requiredBy:
  - src/graph/max_flow/edmonds_karp/edge.rs
  timestamp: '2021-02-08 00:55:24+09:00'
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
