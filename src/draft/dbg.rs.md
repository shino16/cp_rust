---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[cfg(debug_assertions)]\nmacro_rules! dbg {\n\t($($x:expr),*) => { std::dbg!($($x),*)\
    \ }\n}\n\n#[cfg(not(debug_assertions))]\nmacro_rules! dbg {\n\t($($x:expr),*)\
    \ => { std::convert::identity(($($x),*)) }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/dbg.rs
  requiredBy: []
  timestamp: '2020-12-21 20:11:53+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/dbg.rs
layout: document
redirect_from:
- /library/src/draft/dbg.rs
- /library/src/draft/dbg.rs.html
title: src/draft/dbg.rs
---
