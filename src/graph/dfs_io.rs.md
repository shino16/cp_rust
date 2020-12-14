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
  code: "pub use crate::ds::bitset::*;\nuse crate::graph::*;\n\n\npub fn dfs_io<G:\
    \ Graph, FI: FnMut(usize, usize), FO: FnMut(usize, usize)>(\n\tg: &G,\n\ts: usize,\n\
    \tmut fi: FI,\n\tmut fo: FO,\n) {\n\tlet mut visited = new_bitset(g.len());\n\t\
    visited.set_bit(s, true);\n\tlet mut togo = vec![(s, !0)];\n\twhile let Some((v,\
    \ par)) = togo.pop() {\n\t\tif v.get_bit(31) {\n\t\t\tfo(!v, par);\n\t\t} else\
    \ {\n\t\t\tfi(v, par);\n\t\t\ttogo.push((!v, par));\n\t\t\tg.adj(v, |w| {\n\t\t\
    \t\tif visited.modify_bit(w, true) {\n\t\t\t\t\ttogo.push((w, v));\n\t\t\t\t}\n\
    \t\t\t});\n\t\t}\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/dfs_io.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/dfs_io.rs
layout: document
redirect_from:
- /library/src/graph/dfs_io.rs
- /library/src/graph/dfs_io.rs.html
title: src/graph/dfs_io.rs
---
