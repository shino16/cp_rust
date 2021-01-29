---
data:
  _extendedDependsOn: []
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
  code: "// use crate::bound::Bound;\n// pub use crate::num::*;\n\n// #[derive(Clone,\
    \ Copy, Debug)]\n// pub struct Edge<C: INum + Bound> {\n// \tpub to: usize,\n\
    // \tpub cap: C,\n// \trev: usize,\n// }\n\n// /// O(V^2E)\n// #[derive(Clone)]\n\
    // pub struct PushRelabel<C: INum + Bound> {\n// \tpub graph: Vec<Vec<Edge<C>>>,\n\
    // }\n\n// impl<C: INum + Bound> PushRelabel<C> {\n// \tpub fn new(len: usize)\
    \ -> Self {\n// \t\tSelf { graph: vec![Vec::new(); len] }\n// \t}\n// \tpub fn\
    \ len(&self) -> usize {\n// \t\tself.graph.len()\n// \t}\n// \tpub fn from_digraph(graph:\
    \ &[Vec<(usize, C)>]) -> Self {\n// \t\tlet mut ret = Self::new(graph.len());\n\
    // \t\tfor (v, adj) in (0..).zip(graph) {\n// \t\t\tfor &(w, cap) in adj {\n//\
    \ \t\t\t\tret.add_edge(v, w, cap);\n// \t\t\t}\n// \t\t}\n// \t\tret\n// \t}\n\
    // \tpub fn add_vertex(&mut self) -> usize {\n// \t\tself.graph.push(Vec::new());\n\
    // \t\tself.graph.len() - 1\n// \t}\n// \tpub fn add_edge(&mut self, v: usize,\
    \ w: usize, cap: C) {\n// \t\tlet (vidx, widx) = (self.graph[v].len(), self.graph[w].len());\n\
    // \t\tself.graph[v].push(Edge { to: w, cap, rev: widx });\n// \t\tself.graph[w].push(Edge\
    \ { to: v, cap: C::ZERO, rev: vidx });\n// \t}\n// \tpub fn solve(&mut self, s:\
    \ usize, t: usize) -> C {\n// \t\tlet mut height = vec![0; self.len()];\n// \t\
    \theight[s] = self.len();\n// \t\tlet mut excess = vec![C::ZERO; self.len()];\n\
    // \t\texcess[s] = C::MAX;\n// \t\texcess[t] = -C::MAX;\n\n// \t\tlet push = |v:\
    \ usize, e: &mut Edge<C>| {\n\n// \t\t};\n\n// \t\tpanic!(\"{}\", \"\")\n// \t\
    }\n// }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/max_flow/push_relabel.rs
  requiredBy: []
  timestamp: '2021-01-29 12:22:27+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/max_flow/push_relabel.rs
layout: document
redirect_from:
- /library/src/graph/max_flow/push_relabel.rs
- /library/src/graph/max_flow/push_relabel.rs.html
title: src/graph/max_flow/push_relabel.rs
---
