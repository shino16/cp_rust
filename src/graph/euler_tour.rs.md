---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::alg::*;\nuse crate::ds::bitset::*;\npub use crate::graph::*;\n\
    \npub fn euler_tour<A: Group, G: WTree<A::Item>>(g: &G, s: usize, a: A) -> Vec<A::Item>\n\
    where\n\tA::Item: Clone,\n{\n\tlet mut edges = Vec::new();\n\tlet mut togo = vec![(s,\
    \ !0, a.unit())];\n\twhile let Some((v, par, e)) = togo.pop() {\n\t\tif v.get_bit(31)\
    \ {\n\t\t\tedges.push(a.inv(e));\n\t\t} else {\n\t\t\tedges.push(e.clone());\n\
    \t\t\ttogo.push((!v, 0, e.clone()));\n\t\t\tg.adj_w(v, |w, e| {\n\t\t\t\tif w\
    \ != par {\n\t\t\t\t\ttogo.push((w, v, e.clone()));\n\t\t\t\t}\n\t\t\t})\n\t\t\
    }\n\t}\n\tedges\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/euler_tour.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/euler_tour.rs
layout: document
redirect_from:
- /library/src/graph/euler_tour.rs
- /library/src/graph/euler_tour.rs.html
title: src/graph/euler_tour.rs
---
