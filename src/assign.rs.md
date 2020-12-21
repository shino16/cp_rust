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
  code: "pub fn assign_if<T, F: Fn(&T, &T) -> bool>(v: T, var: &mut T, f: F) -> bool\
    \ {\n\tif f(&v, var) {\n\t\t*var = v;\n\t\ttrue\n\t} else {\n\t\tfalse\n\t}\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/assign.rs
  requiredBy: []
  timestamp: '2020-12-21 16:30:24+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/assign.rs
layout: document
redirect_from:
- /library/src/assign.rs
- /library/src/assign.rs.html
title: src/assign.rs
---
