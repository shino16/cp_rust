---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.5/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/pruefer.rs\n"
  code: "pub fn decode(code: &[usize]) -> Vec<(usize, usize)> {\n    let n = code.len()\
    \ + 2;\n    let mut deg = vec![1; n];\n    for &v in code {\n        deg[v] +=\
    \ 1;\n    }\n    let mut i = 0;\n    while deg[i] != 1 {\n        i += 1;\n  \
    \  }\n    let mut leaf = i;\n    let mut edges = Vec::with_capacity(n - 1);\n\
    \    for &v in code {\n        edges.push((leaf, v));\n        deg[v] -= 1;\n\
    \        if deg[v] == 1 && v < i {\n            leaf = v;\n        } else {\n\
    \            i += 1;\n            while deg[i] != 1 {\n                i += 1;\n\
    \            }\n            leaf = i;\n        }\n    }\n    edges.push((leaf,\
    \ n - 1));\n    edges\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/tree/pruefer.rs
  requiredBy: []
  timestamp: '2021-03-06 13:39:22+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/pruefer.rs
layout: document
redirect_from:
- /library/src/graph/tree/pruefer.rs
- /library/src/graph/tree/pruefer.rs.html
title: src/graph/tree/pruefer.rs
---
