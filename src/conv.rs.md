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
  code: "pub trait Conv: Sized {\n\tfn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>)\
    \ -> Vec<Self> {\n\t\tConv::conv_in_place(&mut lhs, &mut rhs);\n\t\tlhs\n\t}\n\
    \tfn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>);\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/conv.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/conv.rs
layout: document
redirect_from:
- /library/src/conv.rs
- /library/src/conv.rs.html
title: src/conv.rs
---
