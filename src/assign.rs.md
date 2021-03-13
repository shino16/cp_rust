---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.2/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/assign.rs\n"
  code: "pub fn assign_if<T, F: Fn(&T, &T) -> bool>(v: T, var: &mut T, f: F) -> bool\
    \ {\n    if f(&v, var) {\n        *var = v;\n        true\n    } else {\n    \
    \    false\n    }\n}\n\npub fn chmin<T: Ord>(v: T, var: &mut T) -> bool {\n  \
    \  assign_if(v, var, |x, y| x < y)\n}\n\npub fn chmax<T: Ord>(v: T, var: &mut\
    \ T) -> bool {\n    assign_if(v, var, |x, y| x > y)\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/assign.rs
  requiredBy:
  - src/graph/dijkstra.rs
  timestamp: '2021-02-13 16:52:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/assign.rs
layout: document
redirect_from:
- /library/src/assign.rs
- /library/src/assign.rs.html
title: src/assign.rs
---
