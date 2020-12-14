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
  code: "pub trait ChMaxMin: Ord + Sized {\n\tfn chmax(&mut self, v: Self) -> bool\
    \ {\n\t\tif *self < v {\n\t\t\t*self = v;\n\t\t\ttrue\n\t\t} else {\n\t\t\tfalse\n\
    \t\t}\n\t}\n\tfn chmin(&mut self, v: Self) -> bool {\n\t\tif *self > v {\n\t\t\
    \t*self = v;\n\t\t\ttrue\n\t\t} else {\n\t\t\tfalse\n\t\t}\n\t}\n}\n\nimpl<T:\
    \ Ord + Sized> ChMaxMin for T {}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ord.rs
  requiredBy: []
  timestamp: '2020-12-15 00:46:43+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ord.rs
layout: document
redirect_from:
- /library/src/ord.rs
- /library/src/ord.rs.html
title: src/ord.rs
---
