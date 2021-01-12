---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/assign.rs\n"
  code: "pub fn assign_if<T, F: Fn(&T, &T) -> bool>(v: T, var: &mut T, f: F) -> bool\
    \ {\n\tif f(&v, var) {\n\t\t*var = v;\n\t\ttrue\n\t} else {\n\t\tfalse\n\t}\n\
    }\n\npub fn chmin<T: Ord>(v: T, var: &mut T) -> bool {\n\tassign_if(v, var, |x,\
    \ y| x < y)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/assign.rs
  requiredBy:
  - src/graph/dijkstra.rs
  timestamp: '2021-01-03 22:19:51+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/assign.rs
layout: document
redirect_from:
- /library/src/assign.rs
- /library/src/assign.rs.html
title: src/assign.rs
---
