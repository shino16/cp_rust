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
  code: "pub use super::*;\nuse crate::ds::bitset::*;\n\n// f: (v, par)\npub fn dfs<G:\
    \ Graph, F: FnMut(usize, usize)>(g: &G, s: usize, mut f: F) {\n\tlet mut togo\
    \ = vec![(s, !0)];\n\tlet mut visited = new_bitset(g.len());\n\tvisited.set_bit(s,\
    \ true);\n\twhile let Some((v, par)) = togo.pop() {\n\t\tf(v, par);\n\t\tg.adj(v,\
    \ |w| {\n\t\t\tif visited.modify_bit(w, true) {\n\t\t\t\ttogo.push((w, v));\n\t\
    \t\t}\n\t\t});\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/dfs.rs
  requiredBy: []
  timestamp: '2020-12-21 16:41:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs.rs
layout: document
redirect_from:
- /library/src/graph/dfs.rs
- /library/src/graph/dfs.rs.html
title: src/graph/dfs.rs
---
