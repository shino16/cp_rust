---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/graph.rs
    title: src/graph.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tsort.rs\n"
  code: "pub use super::*;\n\npub fn topological_sort(g: &impl Graph) -> Option<Vec<usize>>\
    \ {\n    let mut in_deg = vec![0; g.len()];\n    for v in 0..g.len() {\n     \
    \   g.adj(v, |w| in_deg[w] += 1);\n    }\n    let mut stack = Vec::with_capacity(g.len());\n\
    \    for v in 0..g.len() {\n        if in_deg[v] == 0 {\n            stack.push(v);\n\
    \        }\n    }\n    let mut res = Vec::with_capacity(g.len());\n    while let\
    \ Some(v) = stack.pop() {\n        res.push(v);\n        g.adj(v, |w| {\n    \
    \        in_deg[w] -= 1;\n            if in_deg[w] == 0 {\n                stack.push(w);\n\
    \            }\n        });\n    }\n    if res.len() == g.len() { Some(res) }\
    \ else { None }\n}\n"
  dependsOn:
  - src/graph.rs
  isVerificationFile: false
  path: src/graph/tsort.rs
  requiredBy: []
  timestamp: '2021-03-31 15:51:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tsort.rs
layout: document
redirect_from:
- /library/src/graph/tsort.rs
- /library/src/graph/tsort.rs.html
title: src/graph/tsort.rs
---
