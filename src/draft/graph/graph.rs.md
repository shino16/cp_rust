---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::graph::*;\n#[derive(Debug, Default)]\npub struct DiGraph(Vec<Vec<usize>>);\n\
    \nimpl<'g> Graph<'g> for DiGraph {\n\ttype V = usize;\n\ttype E = ();\n\ttype\
    \ VIter = std::iter::Copied<std::slice::Iter<'g, Self::V>>;\n\ttype EIter = std::iter::Take<std::iter::Repeat<()>>;\n\
    \tfn add_edge(&mut self, u: Self::V, v: Self::V, (): Self::E) {\n\t\tself.0[u].push(v);\n\
    \t}\n\tfn adj_v<'a: 'g>(&'a self, v: Self::V) -> Self::VIter {\n\t\tself.0[v].iter().copied()\n\
    \t}\n\tfn adj_e<'a: 'g>(&'a self, v: Self::V) -> Self::EIter {\n\t\tstd::iter::repeat(()).take(self.0[v].len())\n\
    \t}\n\tfn dim(&self) -> Self::V {\n\t\tself.0.len()\n\t}\n}\n\nimpl DiGraph {\n\
    \tpub fn new(n: usize) -> Self {\n\t\tSelf(vec![Vec::new(); n])\n\t}\n}\n\n#[derive(Debug,\
    \ Default)]\npub struct BiDir<G>(G);\n\nimpl<'g, G: Graph<'g>> Graph<'g> for BiDir<G>\
    \ {\n\ttype V = G::V;\n\ttype E = G::E;\n\ttype VIter = G::VIter;\n\ttype EIter\
    \ = G::EIter;\n\tfn add_edge<'a>(&'a mut self, u: Self::V, v: Self::V, e: Self::E)\
    \ {\n\t\tself.0.add_edge(u, v, e);\n\t\tself.0.add_edge(v, u, e);\n\t}\n\tfn adj_v<'a:\
    \ 'g>(&'a self, v: Self::V) -> Self::VIter {\n\t\tself.0.adj_v(v)\n\t}\n\tfn adj_e<'a:\
    \ 'g>(&'a self, v: Self::V) -> Self::EIter {\n\t\tself.0.adj_e(v)\n\t}\n\tfn dim(&self)\
    \ -> Self::V {\n\t\tself.0.dim()\n\t}\n}\n\npub type UGraph = BiDir<DiGraph>;\n\
    \nimpl UGraph {\n\tpub fn new(n: usize) -> Self {\n\t\tSelf(DiGraph::new(n))\n\
    \t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/graph/graph.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/graph/graph.rs
layout: document
redirect_from:
- /library/src/draft/graph/graph.rs
- /library/src/draft/graph/graph.rs.html
title: src/draft/graph/graph.rs
---
